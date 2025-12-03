pub mod method;
pub mod parse_error;
pub mod request;
pub mod response;

pub use method::Method;
pub use parse_error::ParseError;
pub use request::Request;
pub use response::{Response, StatusCode};

/// Handler trait for processing HTTP requests
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
}
