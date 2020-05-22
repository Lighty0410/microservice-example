use crate::model::{GenericSuccess, User, UserLogin, UserResponse};
use crate::server::{json_response::JSON, RequestBody, Router};
use crate::utils;
use anyhow::anyhow;
use hyper::header::{COOKIE, SET_COOKIE};
use hyper::StatusCode;
use hyper::{Body, HeaderMap, Method, Response};

type ResponseBody = anyhow::Result<Response<Body>>;

impl Router {
    pub(super) async fn user_router(&mut self, req: RequestBody) -> ResponseBody {
        match (req.method(), req.uri().path()) {
            (&Method::POST, "/create-user") => self.create_user(req).await,

            (&Method::POST, "/login") => self.login_user(req).await,

            (&Method::POST, "/user") => self.get_user(req).await,

            _ => Err(anyhow!("wrong endpoint")),
        }
    }

    async fn create_user(&mut self, req: RequestBody) -> ResponseBody {
        let user: User = utils::parse_body(req, &mut Vec::<u8>::new()).await?;

        self.controller.create_user(user)?;

        Ok(JSON::response(
            Some(GenericSuccess {
                result: "user created successfully!".into(),
            }),
            None,
            Some(StatusCode::CREATED),
        ))
    }
    //
    async fn login_user(&mut self, req: RequestBody) -> ResponseBody {
        if self.get_user_by_token(&req)?.status().is_success() {
            let mut headers = HeaderMap::new();
            if let Some(token) = Self::token_header_from_cookie(req.headers()) {
                headers.insert(SET_COOKIE, ["token =", token].concat().parse()?);
            }

            return Ok(JSON::response(
                Some(GenericSuccess {
                    result: "already logged in!".into(),
                }),
                Some(headers),
                Some(StatusCode::OK),
            ));
        }

        let login_info: UserLogin = utils::parse_body(req, &mut Vec::<u8>::new()).await?;
        let is_logged_in = self.controller.check_password_create_hash(login_info)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            SET_COOKIE,
            ["token =", is_logged_in.as_str()].concat().parse()?,
        );

        Ok(JSON::response(
            Some(GenericSuccess {
                result: "logged in successfully!".into(),
            }),
            Some(headers),
            Some(StatusCode::OK),
        ))
    }

    async fn get_user(&mut self, req: RequestBody) -> ResponseBody {
        self.get_user_by_token(&req)
    }

    fn get_user_by_token(&mut self, req: &RequestBody) -> ResponseBody {
        let mut get_user_by_token_clos = |token: &str| -> ResponseBody {
            let user_resp = self.controller.get_user_by_token(token)?;

            let mut headers = HeaderMap::new();
            headers.insert(SET_COOKIE, ["token =", token].concat().parse()?);

            let user_response: UserResponse = user_resp.into();

            Ok(JSON::response(
                Some(user_response),
                Some(headers),
                Some(StatusCode::OK),
            ))
        };

        let token_from_cookie = Self::token_header_from_cookie(req.headers());
        let token_from_header = req.headers().get("token");

        match (token_from_cookie, token_from_header) {
            (Some(token), _) => get_user_by_token_clos(token),
            (_, Some(token)) => get_user_by_token_clos(token.to_str()?),
            _ => Err(anyhow!("wrong endpoint")),
        }
    }
    //
    fn token_header_from_cookie(headers: &HeaderMap) -> Option<&str> {
        Option::from(
            headers
                .get(COOKIE)?
                .to_str()
                .ok()?
                .trim_start_matches("token="),
        )
    }
}
