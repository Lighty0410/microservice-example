use crate::model::{User, UserLogin};
use crate::server::{utils, RequestBody, ResponseBody, Router};
use hyper::{Body, Method, Response};
use reqwest::StatusCode;

impl Router {
    pub(super) async fn user_router(&mut self, req: RequestBody) -> Response<Body> {
        let mut response = Response::new(Body::default());

        match (req.method(), req.uri().path()) {
            (&Method::POST, "/create-user") => match self.create_user(req).await {
                Ok(_) => {
                    *response.body_mut() = Body::from("user created succesfully!");
                    *response.status_mut() = StatusCode::CREATED;
                    response
                }
                Err(e) => {
                    *response.body_mut() = Body::from("cannot create user");
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    response
                }
            },

            (&Method::POST, "/login") => match self.login_user(req).await {
                Ok(_) => {
                    *response.body_mut() = Body::from("logged in successfully!");
                    *response.status_mut() = StatusCode::OK;
                    response
                }
                Err(e) => {
                    *response.body_mut() = Body::from(format!("cannot log in: {}", e));
                    *response.status_mut() = StatusCode::BAD_REQUEST;
                    response
                }
            },
            _ => {
                *response.body_mut() = Body::from(format!("unknown error"));
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                response
            }
        }
    }

    pub(super) async fn create_user(&mut self, req: RequestBody) -> Result<(), String> {
        let response = Response::new(Body::empty());
        let user: User = utils::parse_body(req, &mut Vec::<u8>::new()).await?;

        self.controller.create_user(user)?;
        Ok(())
    }

    pub(super) async fn login_user(&mut self, req: RequestBody) -> Result<(), String> {
        let login_info: UserLogin = utils::parse_body(req, &mut Vec::<u8>::new()).await?;

        self.controller.check_password(login_info)
    }
}
