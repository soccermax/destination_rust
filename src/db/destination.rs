use crate::db::client::create_client;
use redis;
use redis::Commands;
use serde_json::{json, Map, Value};
use std::string::ToString;
use uuid::Uuid;

use crate::db::client;
use crate::db::error;
use crate::model::destination::{Authentication, Destination, Protocol};

pub fn create_destination(mut new_destination: Destination) -> Result<Destination, error::DbError> {
    let mut connection = create_client()?;
    let mut all_destinations = get_all()?;
    let existing_destination = &all_destinations[&new_destination.name];
    // let test = get_all_v2();
    if serde_json::Value::is_null(existing_destination) {
        new_destination.id = Some(Uuid::new_v4().to_string());
        all_destinations[&new_destination.name] = serde_json::to_value(&new_destination).unwrap();
    } else {
        return Err(error::DbError::AlreadyExists {
            name: new_destination.name,
        });
    }
    connection.set("DESTINATION", all_destinations.to_string())?;
    Ok(new_destination)
}

pub fn get_all() -> Result<serde_json::Value, error::DbError> {
    let mut con = create_client()?;
    let all_destinations: redis::Value = con.get("DESTINATION").unwrap();

    if all_destinations == redis::Value::Nil {
        return Ok(json!({}));
    }
    let string: String = redis::from_redis_value(&all_destinations).unwrap();
    Ok(serde_json::from_str(&string).unwrap())
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
