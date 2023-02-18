use redis::RedisError;

#[derive(Debug)]
pub enum DbError {
    NotReachable,
    AlreadyExists { name: String },
    NotFound,
    Conflict,
}

impl From<RedisError> for DbError {
    fn from(value: RedisError) -> Self {
        println!("db error: {:?}", value);
        Self::NotReachable {}
    }
}
