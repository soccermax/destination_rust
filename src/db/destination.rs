use crate::db::client::create_client;
use redis;
use redis::{AsyncCommands, Commands};
use serde_json::{Map, Value};
use std::string::ToString;
use uuid::Uuid;

use crate::db::error;
use crate::model::destination::Destination;

pub fn create(mut new_destination: Destination) -> Result<Destination, error::DbError> {
    let mut connection = create_client()?;
    require_lock(&mut connection)?;
    let mut all_destinations = get_all_map(&mut connection)?;
    match all_destinations.contains_key(&new_destination.name) {
        true => {
            release_lock(&mut connection)?;
            Err(error::DbError::AlreadyExists {
                name: new_destination.name,
            })
        }
        false => {
            let destination_name = new_destination.name.to_string();
            new_destination.id = Some(Uuid::new_v4().to_string());
            let value = serde_json::to_value(&new_destination).unwrap();
            all_destinations.insert(new_destination.name, value);
            connection.set(
                "DESTINATION",
                serde_json::to_string(&all_destinations).expect("TODO: panic message"),
            )?;
            let test = all_destinations.remove(&destination_name).unwrap();
            let result: Destination = serde_json::from_value(test).unwrap();
            release_lock(&mut connection)?;
            Ok(result)
        }
    }
}

fn require_lock(con: &mut redis::Connection) -> Result<bool, error::DbError> {
    let test: redis::Value = con.set_nx("DESTINATION_LOCK", "LOCK")?;
    let result: u8 = redis::from_redis_value(&test).unwrap();

    return match result {
        0 => Err(error::DbError::Conflict),
        1 => {
            let result_expire: redis::Value = con.expire("DESTINATION_LOCK", 10)?;
            match redis::from_redis_value(&result_expire).unwrap() {
                1 => Ok(true),
                _ => panic!("the world burns"),
            }
        }
        _ => panic!("the world burns"),
    };
}

fn release_lock(con: &mut redis::Connection) -> Result<(), error::DbError> {
    let test: redis::Value = con.del("DESTINATION_LOCK")?;
    let result: u8 = redis::from_redis_value(&test).unwrap();
    return match result {
        0 => Err(error::DbError::Conflict),
        1 => {
            let result_expire: redis::Value = con.expire("DESTINATION_LOCK", 10)?;
            match redis::from_redis_value(&result_expire).unwrap() {
                0 => Ok(()),
                _ => panic!("the world burns"),
            }
        }
        _ => panic!("the world burns"),
    };
}

fn get_all_map(con: &mut redis::Connection) -> Result<Map<String, Value>, error::DbError> {
    let all_destinations: redis::Value = con.get("DESTINATION")?;
    if all_destinations == redis::Value::Nil {
        return Ok(Map::new());
    }

    let string: String = redis::from_redis_value(&all_destinations).unwrap();
    let result: Value = serde_json::from_str(&string).unwrap();
    let map: Map<String, Value> = result.as_object().unwrap().clone();
    Ok(map)
}

async fn get_all_map_async(
    connection_manager: &mut redis::aio::ConnectionManager,
) -> Result<Map<String, Value>, error::DbError> {
    let all_destinations: redis::Value = connection_manager.get("DESTINATION").await?;
    if all_destinations == redis::Value::Nil {
        return Ok(Map::new());
    }

    let string: String = redis::from_redis_value(&all_destinations).unwrap();
    let result: Value = serde_json::from_str(&string).unwrap();
    let map: Map<String, Value> = result.as_object().unwrap().clone();
    Ok(map)
}

pub fn get_all() -> Result<Vec<Destination>, error::DbError> {
    let mut connection = create_client()?;
    let map = get_all_map(&mut connection)?;
    let destinations = map
        .iter()
        .map(|(_, value)| serde_json::from_value(value.clone()).unwrap())
        .collect();
    Ok(destinations)
}

pub fn get(name: String) -> Result<Destination, error::DbError> {
    let mut connection = create_client()?;
    let mut all_destinations = get_all_map(&mut connection)?;
    match all_destinations.remove(&name) {
        Some(destination) => {
            let test: Destination = serde_json::from_value(destination).unwrap();
            Ok(test)
        }
        None => Err(error::DbError::NotFound),
    }
}

pub async fn getV2(
    mut connection_manager: redis::aio::ConnectionManager,
    name: String,
) -> Result<Destination, error::DbError> {
    let mut all_destinations = get_all_map_async(&mut connection_manager).await?;
    match all_destinations.remove(&name) {
        Some(destination) => {
            let test: Destination = serde_json::from_value(destination).unwrap();
            Ok(test)
        }
        None => Err(error::DbError::NotFound),
    }
}

pub fn delete(name: String) -> Result<(), error::DbError> {
    let mut connection = create_client()?;
    require_lock(&mut connection)?;
    let mut all_destinations = get_all_map(&mut connection)?;
    match all_destinations.contains_key(&name) {
        true => {
            all_destinations.remove(&name);
            let mut connection = create_client()?;
            connection.set(
                "DESTINATION",
                serde_json::to_string(&all_destinations).expect("TODO: panic message"),
            )?;
            release_lock(&mut connection)?;
            Ok(())
        }
        false => {
            release_lock(&mut connection)?;
            Err(error::DbError::NotFound)
        }
    }
}
