mod json_response;
mod user_router;
use crate::controller::Controller;
use hyper::{Body, Request, Response};
use std::convert::Infallible;

type RequestBody = Request<Body>;
type ResponseBody = Result<Response<Body>, Infallible>;

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
    pub async fn new_server(&mut self, req: Request<Body>) -> ResponseBody {
        Ok(self.user_router(req).await)
    }
}
