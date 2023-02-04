use redis;
use redis::{ConnectionLike, RedisError};

// TODO: switch to the async api
pub fn create_client() -> Result<redis::Connection, RedisError> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;

    let result = con.is_open();
    println!("is open: {}", result);
    Ok(con)
}