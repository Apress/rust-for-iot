
// hyper
use hyper::header;
//use hyper::header::AUTHORIZATION;
use hyper::header::{Authorization, Bearer};

use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};
use iron::status::Status;

use crate::errors::{Error, ResultExt, UserResult};
use crate::errors::ErrorKind::JwtValidation;

// jwt
use std::default::Default;
use log::{info, debug, warn};
use std::fmt::{self, Debug};

// tag::authcheck[]
use futures::executor::block_on;

pub struct AuthorizationCheck {
    jwks: JWKS,
    // static and this will never change once set
    auth_url: String
}

impl AuthorizationCheck {
    pub fn new(auth_url: &str) -> AuthorizationCheck {
        // Get the jwks
        let jwks = block_on(jwks_fetching_function(auth_url));
        AuthorizationCheck {
            jwks: jwks,
            auth_url: auth_url.to_string()
        }
    }
}
// end::authcheck[]


// tag::authuser[]
pub struct AuthorizedUser {     // <1>
    user_id: String
}

impl AuthorizedUser {
    pub fn new(user_id: String) -> AuthorizedUser {
        AuthorizedUser {
            user_id: user_id
        }
    }
}

pub struct Value(AuthorizedUser);

impl typemap::Key for AuthorizedUser { type Value = Value; }

impl BeforeMiddleware for AuthorizationCheck {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let access_token = parse_access_token(&req, self.auth_url.as_str()); // <2>
        match  access_token {
            Ok(user_id) => {
                req.extensions.insert::<AuthorizedUser>(Value(AuthorizedUser::new(user_id)));
                Ok(())
            },
            Err(e) => {
                let error = Error::from(JwtValidation(e));                  // <3>
                Err(IronError::new(error, Status::BadRequest))
            }
        }
    }
}

pub trait UserIdRequest {                       // <4>
    fn get_user_id(&self) -> String;
}

impl<'a, 'b> UserIdRequest for Request<'a, 'b> {
    fn get_user_id(&self) -> String {
        let user_value = self.extensions.get::<AuthorizedUser>().chain_err(|| "No user id, this should never happen").unwrap();
        let &Value(ref user) = user_value;
        // Clones it since we want to pass handling of it back
        user.user_id.clone()
    }
}
// end::authuser[]

use serde::{Serialize, Deserialize};
use alcoholic_jwt::{JWKS, Validation, validate, token_kid, ValidJWT};


// tag::parse_token[]
fn parse_access_token(request: &Request,  auth_url: &str) -> UserResult {
    // Get the ful Authorization header from the incoming request headers
    let auth_header = match request.headers.get::<Authorization<Bearer>>() {    // <1>
        Some(header) => header,
        None => panic!("No authorization header found")
    };
    debug!("Auth Header :: {:?}", auth_header);

    let jwt = header::HeaderFormatter(auth_header).to_string();         // <2>
    debug!("JWT :: {:?}", jwt);

    let jwt_slice = &jwt[7..];
    debug!("JWT Slice :: {:?}", jwt_slice);

    let item = block_on(retrieve_user(jwt_slice, auth_url));

    Ok(item.unwrap())
}

#[derive(Deserialize, Debug)]
struct Auth0Result {
    iss: String,
    sub: String,
    aud: String
}

async fn retrieve_user(jwt: &str, auth_url: &str) -> Result<String, reqwest::Error> {
    use std::collections::HashMap;
    use http::{HeaderMap,HeaderValue};

    let url = format!("https://{}/userinfo", auth_url);

    // eaders
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(jwt).unwrap());
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());

    let mut json = reqwest::Client::new()           // <3>
                    .get(&url)
                    .headers(headers)
                    .send()
                    .await?
                    .json::<Auth0Result>()
                    .await?;

    Ok(json.sub)
}
// end::parse_token[]

// tag::user_token[]
fn parse_id_token(jwt_slice: &str, jwks: &JWKS, auth_url: &str) -> UserResult {
    debug!("JWT Slice :: {:?}", jwt_slice);

    // Several types of built-in validations are provided:
    let validations = vec![
        Validation::Issuer(format!("https://{}/", auth_url).into()),    // <1>
        Validation::SubjectPresent,                                     // <2>
        Validation::NotExpired,                                         // <3>
    ];

    let kid = token_kid(&jwt_slice)                     // <4>
        .expect("Failed to decode token headers")
        .expect("No 'kid' claim present in token");

    let jwk = jwks.find(&kid).expect("Specified key not found in set");

    //let result: ValidJWT = validate(jwt_slice, jwk, validations).expect("Token validation has failed!");
    let user_id = validate(jwt_slice, jwk, validations)?        // <5>
        .claims.get("sub").unwrap().to_string();                        // <6>
    Ok(user_id)                                                               // <7>
}

async fn jwks_fetching_function(url: &str) -> JWKS {                                  // <8>
    use std::io::Read;
    use std::collections::HashMap;

    let jwks_json: String = {
        let url_jwks = format!("https://{}/.well-known/jwks.json", url);
        let mut res = reqwest::get(url_jwks.as_str()).await.unwrap();
        res.text().await.unwrap()
    };

    let jwks: JWKS = serde_json::from_str(jwks_json.as_str()).expect("Failed to decode");
    jwks
}
// end::user_token[]

// Awaot Info : https://blog.rust-lang.org/2019/11/07/Async-await-stable.html