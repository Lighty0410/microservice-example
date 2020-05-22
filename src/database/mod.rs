use mongodb::Client;
use thiserror::Error;

mod user;

#[derive(Debug, Clone)]
pub struct UserDB {
    mongo_collection: mongodb::Collection,
    redis_client: redis::Client,
}

#[derive(Debug, Error)]
enum DatabaseError {
    #[error("cannot find user in the database")]
    CannotFindUser,
}

impl UserDB {
    pub fn new(mongo_collection: mongodb::Collection, redis_client: redis::Client) -> Self {
        UserDB {
            mongo_collection,
            redis_client,
        }
    }

    pub fn new_default() -> Self {
        let mongo_collection = build_mongo();
        let redis_client = build_redis();

        UserDB::new(mongo_collection, redis_client)
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
