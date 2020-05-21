use redis::Value::Data;
use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, RedisWrite, ToRedisArgs, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub birthday: String,
    pub gender: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub username: String,
    pub birthday: String,
    pub gender: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericSuccess {
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericError {
    pub reason: String,
    pub caused: String,
}

impl ToRedisArgs for User {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let vector = serde_json::to_vec(self);
        if let Ok(v) = vector {
            out.write_arg(v.as_slice())
        }
    }
}

impl FromRedisValue for User {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Data(data) => {
                let user: User = serde_json::from_slice(data.as_slice()).or_else(|e| {
                    RedisResult::Err(RedisError::from((
                        ErrorKind::TypeError,
                        "failed to parse user",
                        e.to_string(),
                    )))
                })?;
                RedisResult::Ok(user)
            }
            _ => RedisResult::Err(RedisError::from((
                ErrorKind::TypeError,
                "failed to parse user",
            ))),
        }
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            username: user.username,
            birthday: user.birthday,
            gender: user.gender,
        }
    }
}
