use crate::database;
mod user;
mod utils;

#[derive(Debug, Clone)]
pub struct Controller {
    pub db: database::UserDB,
}

impl Controller {
    pub fn new(user_db: database::UserDB) -> Self {
        Controller { db: user_db }
    }
}
