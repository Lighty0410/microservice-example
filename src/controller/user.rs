use super::utils as local_utils;
use super::Controller;
use crate::model::User;
use crate::model::UserLogin;
use anyhow::{anyhow, Result};

impl Controller {
    pub fn create_user(&mut self, mut user: User) -> Result<()> {
        local_utils::encode_string_base64(&mut user.password);
        self.db.create_user_mongo(user)
    }

    pub fn check_password_create_hash(&mut self, user_login: UserLogin) -> Result<String> {
        let user = self.db.get_user_by_username_mongo(&user_login.username)?;
        let decoded_password = local_utils::decode_string_base64(&user.password)?;

        if user_login.password != decoded_password {
            return Err(anyhow!("incorrect password"));
        }

        let token = local_utils::generate_token();
        self.db.save_user_by_token(&token, user)?;
        Ok(token)
    }

    pub fn get_user_by_token(&mut self, token: &str) -> Result<User> {
        self.db.get_user_by_token(token)
    }
}
