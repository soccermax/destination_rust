use crate::db::client;
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

pub async fn create_connection_manager() -> Result<redis::aio::ConnectionManager, RedisError> {
    let client2 = redis::Client::open("redis://127.0.0.1/")?;
    let connection = client2.get_tokio_connection_manager().await?;

    // let result = connection.is_open().await;
    // println!("is open: {}", result);
    Ok(connection)
}
