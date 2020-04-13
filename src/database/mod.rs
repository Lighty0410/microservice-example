use crate::model::User;
use mongodb::{options::ClientOptions, Client, Collection};
mod add_new_user;

#[derive(Clone)]
pub struct UserCollection(Collection);

impl UserCollection {
    pub fn new() -> Self {
        let client = Client::with_uri_str("mongodb://localhost:27017/").unwrap();

        let db = client.database("user_db");
        let coll = db.collection("user_collection");
        UserCollection(coll)
    }
}
