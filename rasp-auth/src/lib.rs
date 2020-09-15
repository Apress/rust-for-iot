///
/// I updated yup-auth2 and changed "code" to "device_code" this should be parameter
/// based but not for now
///
/// Right now its a fake runs 3 times then continues.
/// // tag::trait[]
use yup_oauth2::{self, ApplicationSecret, DeviceFlowAuthenticator};                 // <1>
use yup_oauth2::authenticator_delegate::{DeviceAuthResponse, DeviceFlowDelegate};
use log::{info, warn};
// Used to pin that data to a point in memory, makes sure its a stable memory location
use std::pin::Pin;
use std::future::Future;

// Always store to this location
const JSON_SECRET_LOCATION: &str = "tokenstorage.json"; // <2>
// Probably should be a command line parameter
const PROJECT_ID: &str = "rustfortheiot";           // <3>

pub trait VisualDisplay {           // <4>
    fn clear(&mut self);
    fn display_text(&mut self, text: &str);
    fn display_processing(&mut self);
}
// end::trait[]

// tag::access[]
pub struct Access<VD> {
    client_id: String,
    client_secret: String,
    url: String,
    output: Arc<Mutex<VD>>
}
// end::access[]

// tag::access2[]
impl<VD> Access<VD>
    where
        VD: VisualDisplay + Send + Clone + 'static // <1>
{
    pub fn new(client_id: String, client_secret: String, url: String, output: Arc<Mutex<VD>>) -> Access<VD> { // <2>
        // retrieve the access tokens if they exist
        Access {
            client_id: client_id,
            client_secret: client_secret,
            url: url,
            output: output
        }
    }
// end::access2[]

    // tag::auth[]
    pub async fn authenticate(&self) -> bool
        where VD: VisualDisplay + Send + Sync {    // <1>
        // Trait needed for the futures use

        info!("Authenticate");
        // Create our application secret
        let application_secret = ApplicationSecret { // <2>
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            token_uri: format!("https://{}/oauth/token", self.url),
            auth_uri: format!("https://{}/authorize", self.url),
            redirect_uris: vec![],
            project_id: Some(PROJECT_ID.to_string()),   // projectId
            client_email: None,   // clientEmail
            auth_provider_x509_cert_url: None,   // X509 cert auth provider
            client_x509_cert_url: None,   // X509 cert provider
        };

        // Create the flow delegate
        let flow_delegate = Auth0FlowDelegate {     // <3>
            output: self.output.clone()
        };

        let auth = DeviceFlowAuthenticator::builder(application_secret) // <4>
            .flow_delegate(Box::new(flow_delegate))
            .device_code_url(format!("https://{}/oauth/device/code", self.url))
            .persist_tokens_to_disk(JSON_SECRET_LOCATION)
            .grant_type("urn:ietf:params:oauth:grant-type:device_code")
            .build()
            .await
            .expect("authenticator");

        // Set our scopes of data we want to obtain
        let scopes = &["offline_access", "openid", "profile", "email"];  // <5>

        match auth.token(scopes).await {                                            // <6>
            Err(e) => warn!("error: {:?}", e),
            Ok(t) => info!("token: {:?}", t),
        }
        // Unblocked now, let's blank out before we return
        let mut output_ctrls = self.output.lock().unwrap();       // <7>
        output_ctrls.clear();
        true
    }
    // end::auth[]
}

// tag::auth_flow[]
use std::sync::{Arc, Mutex};

// Flow Delegate requires a Clone
#[derive(Clone)]
pub struct Auth0FlowDelegate<VD> {
    output: Arc<Mutex<VD>>
}
// end::auth_flow[]

// tag::auth_impl[]
impl<VD> DeviceFlowDelegate for Auth0FlowDelegate<VD> // <1>
    where
        VD: VisualDisplay + Send + Sync   // <2>
{
    /// Display to the user the instructions to use the user code
    fn present_user_code<'a>(                           // <3>
        &'a self,
        resp: &'a DeviceAuthResponse,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(present_user_code(&self.output, resp))
    }
}
// end::auth_impl[]

// tag::auth_impl2[]
async fn present_user_code<VD>(output: &Arc<Mutex<VD>>, resp: &DeviceAuthResponse)
    where
        VD: VisualDisplay {
    use chrono::Local;

    info!("Please enter {} at {} and grant access to this application", resp.user_code, resp.verification_uri); // <1>
    info!("You have time until {}.", resp.expires_at.with_timezone(&Local));
    // Push to the ED Display
    let mut output_unwrap = output.lock().unwrap();  // <2>
    let text = format!("> {}  ", resp.user_code);

    // Bit of a fake since it will stop processing after this function
    output_unwrap.display_text(text.as_str());       // <3>
    output_unwrap.display_processing();         // <4>
}

// end::auth_impl2[]