use crate::database::UserDB;
use crate::model::User;
use crate::utils::redis_to_struct;
use bson::{doc, DecoderResult};
use mongodb::options::FindOneOptions;
use redis::{Commands, RedisError, RedisResult};

impl UserDB {
    pub fn create_user_mongo(&self, user: User) -> Result<(), String> {
        self.mongo_collection
            .insert_one(
                doc! {"username":user.username,
                "password":user.password, "gender":user.gender, "birthday":user.birthday },
                None,
            )
            .or_else(|e| Err(format!("cannot create user to mongo: {:?}", e)))?;
        Ok(())
    }

    pub fn save_user_by_token(&mut self, token: &str, user: User) -> Result<(), String> {
        let mut redis_connection = self
            .redis_client
            .get_connection()
            .or_else(|e| Err(format!("cannot connect to redis: {:?}", e)))?;

        redis_connection
            .set(token, user)
            .or_else(|e| Err(format!("cannot save user to redis: {}", e)))?;
        Ok(())
    }

    pub fn get_user_by_token(&mut self, token: &str) -> Result<User, String> {
        let mut redis_connection = self
            .redis_client
            .get_connection()
            .or_else(|e| Err(format!("cannot connect to redis: {:?}", e)))?;

        let result: Vec<u8> = redis_connection
            .get(token)
            .or_else(|e| Err(format!("cannot get user by token from redis: {:?}", e)))?;

        redis_to_struct(&result)
    }

    pub fn get_user_by_username_mongo(&self, username: &str) -> Result<User, String> {
        let filter = doc! {"username": username };
        let password = self
            .mongo_collection
            .find_one(filter, FindOneOptions::default())
            .or_else(|e| Err(format!("cannot find user: {:?}", e)))?
            .ok_or_else(|| "cannot find user in the database")?;

        bson::from_bson(bson::Bson::Document(password))
            .or_else(|e| Err(format!("cannot find decode user: {:?}", e)))?
    }
}
