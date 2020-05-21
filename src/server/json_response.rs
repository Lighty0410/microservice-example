use crate::model::GenericError;
use crate::utils::struct_to_json;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::StatusCode;
use hyper::{Body, HeaderMap, Response};
use serde::Serialize;

pub(super) struct JSON;

impl JSON {
    pub(super) fn response<T: Serialize>(
        payload: Option<T>,
        headers: Option<HeaderMap<HeaderValue>>,
        status_code: Option<StatusCode>,
    ) -> Response<Body> {
        let mut response = Response::new(Body::default());
        let default_status = StatusCode::INTERNAL_SERVER_ERROR;
        *response.status_mut() = default_status;

        if let Some(pay) = payload {
            match struct_to_json(pay) {
                Ok(body) => {
                    *response.body_mut() = Body::from(body);
                }
                Err(e) => {
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    println!("cannot encode JSON: {}", e);
                    return response;
                }
            };
        }

        if let Some(headers) = headers {
            for (header, value) in headers {
                if let Some(header) = header {
                    response.headers_mut().insert(header, value);
                }
            }
        }
        response
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(status_code) = status_code {
            *response.status_mut() = status_code;
        }

        response
    }

    pub(super) fn error_parse_body(e: String) -> Response<Body> {
        let json_body = GenericError {
            reason: "incorrect body".into(),
            caused: e,
        };
        let status_code = StatusCode::BAD_REQUEST;

        JSON::response(Some(json_body), None, Some(status_code))
    }
}
