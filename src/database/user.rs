use crate::database::DatabaseError;
use crate::database::UserDB;
use crate::model::User;
use crate::utils::redis_to_struct;

use anyhow::Result;
use bson::doc;
use mongodb::options::FindOneOptions;
use redis::Commands;

impl UserDB {
    pub fn create_user_mongo(&self, user: User) -> Result<()> {
        self.mongo_collection.insert_one(
            doc! {"username":user.username,
            "password":user.password, "gender":user.gender, "birthday":user.birthday },
            None,
        )?;
        Ok(())
    }

    pub fn save_user_by_token(&mut self, token: &str, user: User) -> Result<()> {
        let mut redis_connection = self.redis_client.get_connection()?;

        redis_connection.set(token, user)?;
        Ok(())
    }

    pub fn get_user_by_token(&mut self, token: &str) -> Result<User> {
        let mut redis_connection = self.redis_client.get_connection()?;

        let result: Vec<u8> = redis_connection.get(token)?;

        redis_to_struct(&result)
    }

    pub fn get_user_by_username_mongo(&self, username: &str) -> Result<User> {
        let filter = doc! {"username": username };
        let password = self
            .mongo_collection
            .find_one(filter, FindOneOptions::default())?
            .ok_or(DatabaseError::CannotFindUser)?;

        Ok(bson::from_bson(bson::Bson::Document(password))?)
    }
}
