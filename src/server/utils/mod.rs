use hyper::{Body, HeaderMap, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};

pub(in crate::server) async fn parse_body<'a, T>(
    req: Request<Body>,
    buffer: &'a mut Vec<u8>,
) -> Result<T, String>
where
    T: Deserialize<'a>,
{
    let body = hyper::body::to_bytes(req.into_body())
        .await
        .or_else(|err| Err(format!("cannot read body: {}", err)))?;

    *buffer = body.iter().cloned().collect::<Vec<u8>>();

    serde_json::from_slice::<T>(buffer)
        .or_else(|err| Err(format!("cannot parse body into json: {}", err)))
}
