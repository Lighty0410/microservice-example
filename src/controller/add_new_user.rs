use crate::controller::Controller;
use crate::model::User;

impl Controller {
    pub fn create_user(&self, user: User) {
        self.db.create_user(user);
    }
}
