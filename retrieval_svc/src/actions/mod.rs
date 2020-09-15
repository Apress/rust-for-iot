
macro_rules! controller_imports {
	($($includes:ident),*) => {
        use iron::{Request, Response, IronResult};
        use iron::prelude::*;
        use iron::status;
        use router::Router;
	}
}


pub mod media_data;
pub mod comment;