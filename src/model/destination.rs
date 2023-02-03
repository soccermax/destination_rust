use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Destination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub protocol: Protocol,
    pub port: i32,
    pub url: String,
    pub authentication: Option<Authentication>
}

#[derive(Serialize, Deserialize)]
pub enum Protocol {
  HTTP
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Authentication {
    BasicAuth,
    OAuth2
}