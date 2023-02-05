use crate::db::client::create_client;
use redis;
use redis::Commands;
use serde_json::{Map, Value};
use std::string::ToString;
use uuid::Uuid;

use crate::db::error;
use crate::model::destination::Destination;

pub fn create_destination(mut new_destination: Destination) -> Result<Destination, error::DbError> {
    let mut connection = create_client()?;
    let mut all_destinations = get_all_map()?;
    match all_destinations.get(&new_destination.name) {
        Some(_) => Err(error::DbError::AlreadyExists {
            name: new_destination.name,
        }),
        None => {
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
            Ok(result)
        }
    }
}

fn get_all_map() -> Result<Map<String, Value>, error::DbError> {
    let mut con = create_client()?;
    let all_destinations: redis::Value = con.get("DESTINATION").unwrap();

    if all_destinations == redis::Value::Nil {
        return Ok(Map::new());
    }

    let string: String = redis::from_redis_value(&all_destinations).unwrap();
    let result: Value = serde_json::from_str(&string).unwrap();
    let map: Map<String, Value> = result.as_object().unwrap().clone();
    Ok(map)
}

pub fn get_all() -> Result<Vec<Destination>, error::DbError> {
    let map = get_all_map()?;
    let destinations = map
        .iter()
        .map(|(_, value)| serde_json::from_value(value.clone()).unwrap())
        .collect();
    Ok(destinations)
}

pub fn get_destination(name: String) -> Result<Destination, error::DbError> {
    let mut con = create_client()?;
    let all_destinations: redis::Value = con.get("DESTINATION").unwrap();

    if all_destinations == redis::Value::Nil {
        return Err(error::DbError::NotFound);
    }
    let string: String = redis::from_redis_value(&all_destinations).unwrap();
    let result: Value = serde_json::from_str(&string).unwrap();
    let map: &Map<String, Value> = result.as_object().unwrap();

    return match map.get(&name) {
        Some(destination_string) => {
            let test: Destination = serde_json::from_value(destination_string.clone()).unwrap();
            Ok(test)
        }
        None => Err(error::DbError::NotFound),
    };
}
