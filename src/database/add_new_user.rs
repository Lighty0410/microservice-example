use crate::database::UserCollection;
use crate::model::User;
use bson::{doc, Bson};

impl UserCollection {
    pub fn create_user(&self, user: User) {
        self.0
            .insert_one(
                doc! {"username":user.username,
                "password":user.password, "gender":user.gender, "birthday":user.birthday },
                None,
            )
            .unwrap();
    }
}
