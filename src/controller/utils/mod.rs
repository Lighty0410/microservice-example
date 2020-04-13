use base64;
use std::str;

pub(super) fn encode_password(password: &String) -> String {
    base64::encode(password)
}

pub(super) fn decode_password(password: &String) -> Result<String, String> {
    let decoded =
        base64::decode(password).or_else(|err| Err(format!("cannot decode password :{}", err)))?;

    let password = str::from_utf8(&decoded)
        .or_else(|err| Err(format!("cannot decode password from vec[u8] :{}", err)))?
        .to_string();

    Ok(password)
}
