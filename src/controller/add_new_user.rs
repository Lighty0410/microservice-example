use super::utils as ctrl_utils;
use super::Controller;
use crate::model::User;
use crate::model::UserLogin;
use crate::utils;

impl Controller {
    pub fn create_user(&mut self, mut user: User) -> Result<(), String> {
        user.password = ctrl_utils::encode_password(&user.password);

        let password = user.password.clone();
        let username = user.username.clone();

        self.db.create_user_mongo(user);
        self.db.create_user_redis(username, password)
    }

    pub fn check_password(&mut self, user_login: UserLogin) -> Result<(), String> {
        let password = self.db.get_password(&user_login.username)?;
        let decoded_password = ctrl_utils::decode_password(&password)?;

        if decoded_password != user_login.password {
            return Err(format!("incorrect password"));
        }

        Ok(())
    }
}
