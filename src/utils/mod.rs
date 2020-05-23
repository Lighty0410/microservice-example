use anyhow::Result;
use hyper::{Body, Request};
use serde::{Deserialize, Serialize};

pub(super) async fn parse_body<'a, T: Deserialize<'a>>(
    req: Request<Body>,
    buffer: &'a mut Vec<u8>,
) -> Result<T> {
    let body = hyper::body::to_bytes(req.into_body()).await?;

    *buffer = body.iter().cloned().collect::<Vec<u8>>();

    Ok(serde_json::from_slice::<T>(buffer)?)
}

pub fn struct_to_json<T: Serialize>(some_struct: T) -> Result<String> {
    Ok(serde_json::to_string(&some_struct)?)
}

pub fn redis_to_struct<'a, T: Deserialize<'a>>(buffer: &'a [u8]) -> Result<T> {
    Ok(serde_json::from_slice::<T>(buffer)?)
}
