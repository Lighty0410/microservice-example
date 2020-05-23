mod json_response;
mod macros;
mod user_router;
use crate::controller::Controller;
use crate::model::GenericError;
use crate::server::json_response::JSON;
use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;
use thiserror::Error;

type RequestBody = Request<Body>;
type ResponseBody = anyhow::Result<Response<Body>>;
type ServerResponse = Result<Response<Body>, Infallible>;

// TODO: i wonder if we can use our custom enums in match-{}-arm ???
#[derive(Error, Debug)]
enum ServerErrors {
    // #[error("")]
    // TokenNotFound,
    #[error("path not found")]
    PathNotFound,
}

#[derive(Debug, Clone)]
pub struct Router {
    controller: Controller,
}

impl Router {
    pub fn new(controller: Controller) -> Self {
        Router { controller }
    }
}

impl Router {
    pub async fn new_server(&mut self, req: Request<Body>) -> ServerResponse {
        match self.user_router(req).await {
            Ok(body) => Ok(body),
            Err(err) => Ok(JSON::response(
                Some(GenericError {
                    reason: "error handling the request".to_string(),
                    caused: err.to_string(),
                }),
                None,
                Some(StatusCode::BAD_REQUEST),
            )),
        }
    }
}
