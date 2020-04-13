use super::UserDB;
use redis::{Commands, RedisError};

impl UserDB {
    pub fn get_password(&mut self, username: &String) -> Result<String, String> {
        let password: String = self
            .redis_connection
            .get(username)
            .or_else(|e| Err(format!("cannot get password from redis: {}", e)))?;
        Ok(password)
    }
}
