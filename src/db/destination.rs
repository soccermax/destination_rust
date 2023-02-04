use std::string::ToString;
use redis;
use redis::Commands;
use crate::db::client::create_client;
use serde_json::json;
use uuid::Uuid;
use anyhow;

use super::super::model::destination::{Destination, Authentication, Protocol};
use super::client;

pub fn create_destination(mut new_destination: Destination)
                          -> anyhow::Result<Destination> {
    let mut connection = client::create_client()?;

    let mut all_destinations = get_all()?;
    println!("positive request: {}", all_destinations);

    let existing_destination = &all_destinations[&new_destination.name];
    println!("existing destination: {}", existing_destination);

    if serde_json::Value::is_null(existing_destination) {
        new_destination.id = Some(Uuid::new_v4().to_string());
        all_destinations[&new_destination.name] = serde_json::to_value(&new_destination).unwrap();
    } else {
        anyhow::bail!("it failed!")
    }

    connection.set("DESTINATION", all_destinations.to_string())?;

    Ok(Destination {
        id: None,
        authentication: Some(Authentication::BasicAuth),
        name: String::from("test"),
        protocol: Protocol::Http,
        port: 8080,
        url: String::from("test"),
    })
}

pub fn get_all() -> anyhow::Result<serde_json::Value> {
    let mut con = create_client()?;
    // let mut con = match connection {
    //     Some(c) => c,
    //     None => create_client()?
    // };
    let all_destinations: redis::Value = con.get("DESTINATION")?;

    if all_destinations == redis::Value::Nil {
        return Ok(json!({}));
    }
    let string: String = redis::from_redis_value(&all_destinations)?;
    Ok(serde_json::from_str(&string).unwrap())
}


// use std::error::Error;
// use serde::{Deserialize, Serialize};
// use serde_json;
// use redis;
// use redis::{ConnectionLike, RedisError, RedisResult};
// use redis::Commands;
//
//
// #[derive(Serialize, Deserialize)]
// struct Destination {
//     id: String,
//     name: String,
//     port: u16
// }
//
// fn main() {
//     // Statements here are executed when the compiled binary is called
//     let new_destination = Destination {
//         id: String::from("id"),
//         name: String::from("ER9"),
//         port: 8080
//     };
//     let json_string = serde_json::to_string(&new_destination);
//     // println!("serialized json: {}", &json_string.unwrap());
//
//     let client = redis::Client::open("redis://127.0.0.1/").unwrap();
//     let mut con = client.get_connection().unwrap();
//
//     let result = con.is_open();
//     println!("is open: {}", result);
//     let _:() = con.set(new_destination.id.to_string(), json_string.unwrap()).unwrap();
//     // match result {
//     //     Ok(value) => println!("positive: {}", "done"),
//     //     Err(err) => panic!("panic")
//     // }
//
//
//     let value_get : Result<String, RedisError> = con.get("id");
//     if value_get.is_ok() {
//         let value : String = value_get.unwrap();
//         println!("found the key and all good: {}", &value);
//         let retrieved_destination : Destination = serde_json::from_str(&value).unwrap();
//         println!("destination name is: {} with port {}", retrieved_destination.name,
//                  retrieved_destination.port)
//     } else {
//         println!("key not found");
//     }
//
// }