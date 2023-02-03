use std::error::Error;
use redis;
use redis::{ConnectionLike, RedisError, RedisResult};
use redis::Commands;


// TODO: switch to the async api
pub fn create_client() -> Result<redis::Connection, RedisError> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    let result = con.is_open();
    println!("is open: {}", result);

    return Ok(con);
}
