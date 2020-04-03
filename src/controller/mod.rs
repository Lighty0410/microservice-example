use crate::database;

mod add_new_user;

pub struct Controller {
    pub db: database::UserCollection,
}

impl Controller {
    pub fn new(db: database::UserCollection) -> Self {
        Controller { db }
    }
}
