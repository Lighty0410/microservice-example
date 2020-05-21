use crate::model::{GenericError, GenericSuccess, User, UserLogin, UserResponse};
use crate::server::{json_response::JSON, RequestBody, Router};
use crate::utils;
use hyper::header::{COOKIE, SET_COOKIE};
use hyper::StatusCode;
use hyper::{Body, HeaderMap, Method, Response};
use reqwest::header::HeaderName;
use std::borrow::Borrow;

impl Router {
    pub(super) async fn user_router(&mut self, req: RequestBody) -> Response<Body> {
        // println!("Icnomming request {} {}", req.method(), req.uri().path());
        match (req.method(), req.uri().path()) {
            (&Method::POST, "/create-user") => self.create_user(req).await,

            (&Method::POST, "/login") => self.login_user(req).await,

            (&Method::POST, "/user") => self.get_user(req).await,

            _ => JSON::response(
                Some(GenericError {
                    reason: "unknown endpoint".into(),
                    caused: "wrong redirection".into(),
                }),
                None,
                None,
            ),
        }
    }

    pub(super) async fn create_user(&mut self, req: RequestBody) -> Response<Body> {
        let user: User = match utils::parse_body(req, &mut Vec::<u8>::new()).await {
            Ok(val) => val,
            Err(e) => return JSON::error_parse_body(e),
        };

        self.controller.create_user(user); // TODO: remove unwrap from mongo and handle the error correctly.

        JSON::response(
            Some(GenericSuccess {
                result: "user created successfully!".into(),
            }),
            None,
            Some(StatusCode::CREATED),
        )
    }

    pub(super) async fn login_user(&mut self, req: RequestBody) -> Response<Body> {
        if self.get_user_by_token(&req).status().is_success() {
            let mut headers = HeaderMap::new();
            if let Some(token) = Router::token_header_from_cookie(req.headers()) {
                headers.insert(SET_COOKIE, ["token =", token].concat().parse().unwrap());
            }

            return JSON::response(
                Some(GenericSuccess {
                    result: "already logged!".into(),
                }),
                Some(headers),
                Some(StatusCode::OK),
            );
        }

        let login_info: UserLogin = match utils::parse_body(req, &mut Vec::<u8>::new()).await {
            Ok(val) => val,
            Err(e) => return JSON::error_parse_body(e),
        };

        let is_logged_in = self.controller.check_password_create_hash(login_info);
        match is_logged_in {
            Ok(token) => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    SET_COOKIE,
                    ["token =", token.as_str()].concat().parse().unwrap(),
                );

                JSON::response(
                    Some(GenericSuccess {
                        result: "logged in successfully!".into(),
                    }),
                    Some(headers),
                    Some(StatusCode::OK),
                )
            }
            Err(e) => JSON::response(
                Some(GenericError {
                    reason: "Cannot log in".into(),
                    caused: e,
                }),
                None,
                Some(StatusCode::BAD_REQUEST),
            ),
        }
    }

    pub(super) async fn get_user(&mut self, req: RequestBody) -> Response<Body> {
        self.get_user_by_token(&req)
    }

    fn get_user_by_token(&mut self, req: &RequestBody) -> Response<Body> {
        let mut get_user_by_token_clos = |token: &str| -> Response<Body> {
            let user_resp = self.controller.get_user_by_token(token.borrow());
            match user_resp {
                Ok(user) => {
                    let mut headers = HeaderMap::new();
                    headers.insert(SET_COOKIE, ["token =", token].concat().parse().unwrap());
                    let user_response: UserResponse = user.into();
                    JSON::response(Some(user_response), Some(headers), Some(StatusCode::OK))
                }
                Err(e) => JSON::response(
                    Some(GenericError {
                        reason: "Cannot log in".into(),
                        caused: e,
                    }),
                    None,
                    Some(StatusCode::BAD_REQUEST),
                ),
            }
        };
        let token_from_cookie = Router::token_header_from_cookie(req.headers());
        let token_from_header = req.headers().get("token");
        match (token_from_cookie, token_from_header) {
            (Some(token), _) => get_user_by_token_clos(token),
            (_, Some(token)) => get_user_by_token_clos(token.to_str().unwrap()),
            _ => JSON::response(
                Some(GenericError {
                    reason: "token wasn't found".into(),
                    caused: "token".to_string(),
                }),
                None,
                Some(StatusCode::BAD_REQUEST),
            ),
        }
    }

    fn token_header_from_cookie(headers: &HeaderMap) -> Option<&str> {
        Option::from(
            headers
                .get(COOKIE)?
                .to_str()
                .unwrap()
                .trim_start_matches("token="),
        )
    }
}
