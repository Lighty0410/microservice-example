use crate::model;
use crate::server::{utils, Router};
use hyper::{Body, HeaderMap, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};

impl Router {
    pub(super) async fn login_user(&self, req: Request<Body>) -> Result<Response<Body>, String> {
        let response = Response::new(Body::empty());
        let user: model::User = utils::parse_body(req, &mut Vec::<u8>::new()).await?;

        self.controller.create_user(user);

        Ok(response)
    }
}
