use crate::database::UserDB;
use crate::model::User;
use bson::doc;
use redis::Commands;

impl UserDB {
    pub fn create_user_mongo(&self, user: User) {
        self.mongo_collection
            .insert_one(
                doc! {"username":user.username,
                "password":user.password, "gender":user.gender, "birthday":user.birthday },
                None,
            )
            .unwrap();
    }

    pub fn create_user_redis(&mut self, username: String, password: String) -> Result<(), String> {
        let add_user_res: Result<(), redis::RedisError> =
            self.redis_connection.set(username, password);
        match add_user_res {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("cannot add user to redis: {}", e)),
        }
    }
}
