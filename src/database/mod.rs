mod add_new_user;
mod get_user_password;

use mongodb::Client;

pub struct UserDB {
    mongo_collection: mongodb::Collection,
    redis_connection: redis::Connection,
}

impl UserDB {
    pub fn new(mongo_collection: mongodb::Collection, redis_client: redis::Client) -> Self {
        let redis_connection = redis_client.get_connection().unwrap();

        UserDB {
            mongo_collection,
            redis_connection,
        }
    }
}

pub fn build_redis() -> redis::Client {
    redis::Client::open("redis://127.0.0.1/").unwrap()
}

pub fn build_mongo() -> mongodb::Collection {
    let mongo_client = Client::with_uri_str("mongodb://localhost:27017/").unwrap();
    let db = mongo_client.database("user_db");

    db.collection("user_collection")
}
