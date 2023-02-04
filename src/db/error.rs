use redis::RedisError;
use crate::db::error::DbError::NotReachable;

#[derive(Debug)]
pub enum DbError {
    NotReachable { },
    AlreadyExists { name: String }
}

impl From<RedisError> for DbError {
    fn from(value: RedisError) -> Self {
        println!("db error: {:?}", value);
        NotReachable {}
    }
}