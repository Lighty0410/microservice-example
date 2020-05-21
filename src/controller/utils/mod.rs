use rand::distributions::Alphanumeric;
use rand::Rng;
use std::str;

pub(super) fn encode_string_base64(password: &mut String) {
    let for_encode = password.clone();
    *password = base64::encode(for_encode)
}

pub(super) fn decode_string_base64(password: &str) -> Result<String, String> {
    let decoded =
        base64::decode(password).or_else(|err| Err(format!("cannot decode password :{}", err)))?;

    let password = str::from_utf8(&decoded)
        .or_else(|err| Err(format!("cannot decode password from vec[u8] :{}", err)))?
        .to_string();

    Ok(password)
}

pub(super) fn generate_token() -> String {
    let mut token = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(7)
        .collect::<String>();
    encode_string_base64(&mut token);

    token
}
