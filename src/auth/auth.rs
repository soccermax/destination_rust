use super::app_context::AppContext;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub ext_attr: CustomAttributes,
    pub exp: usize,
    pub authorities: Vec<String>,
    pub scope: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomAttributes {
    pub subaccountid: String,
    pub zdn: String,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
}

#[derive(Serialize, Deserialize)]
struct UaaResponse {
    keys: Vec<KeysUaa>,
}

#[derive(Serialize, Deserialize)]
struct KeysUaa {
    value: String,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Email: {}", self.sub)
    }
}

struct Keys {
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            decoding: DecodingKey::from_rsa_pem(secret).unwrap(),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let mut test = AppContext::from_ref(state);
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data

        let token = test.get_uaa_public_cert().await;

        let keys = Keys::new(token.as_bytes());

        let token_data_result = decode::<Claims>(
            bearer.token(),
            &keys.decoding,
            &Validation::new(Algorithm::RS256),
        );

        match token_data_result {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => {
                println!("could not parse token: {:?}", err);
                Err(AuthError::InvalidToken)
            }
        }
        // Ok(token_data.claims)
    }
}

pub async fn get_public_uaa_pem() -> String {
    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    let token_address = "tokenUrl";
    let req = hyper::Request::builder()
        .method(hyper::Method::GET)
        .uri(token_address)
        .header("user-agent", "the-awesome-agent/007")
        .body(hyper::Body::from(""))
        .unwrap();

    // Pass our request builder object to our client.
    let resp = client.request(req).await.unwrap();

    // Get the response body bytes.
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let string = &String::from_utf8(body_bytes.to_vec()).unwrap();
    let uaa_response: UaaResponse = serde_json::from_str(string).unwrap();
    uaa_response.keys[0].value.to_string()
}
