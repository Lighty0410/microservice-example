use crate::database;

mod add_new_user;
mod utils;

pub struct Controller {
    pub db: database::UserDB,
}

impl Controller {
    pub fn new(mongo_collection: mongodb::Collection, redis_client: redis::Client) -> Self {
        let db = database::UserDB::new(mongo_collection, redis_client);
        Controller { db }
    }
}
