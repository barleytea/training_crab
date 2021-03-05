use jsonwebtoken::{decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::configuration::FirebaseConfig;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub iss: String,
    pub sub: String,
    pub uid: Option<String>,
}

pub async fn verify(
    token: &str,
    firebase_config: &FirebaseConfig,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let kid = match decode_header(token).map(|header| header.kid) {
        Ok(Some(k)) => k,
        Ok(None) => {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::__Nonexhaustive,
            ))
        }
        Err(err) => return Err(err),
    };

    let jwks = get_firebase_jwks(&firebase_config).await.unwrap();
    let jwk = jwks.get(&kid).unwrap();

    let project_id = &firebase_config.project_id;
    let mut validation = Validation {
        iss: Some("https://securetoken.google.com/".to_string() + project_id),
        ..Validation::new(Algorithm::RS256)
    };
    // validate: aud
    validation.set_audience(&[project_id]);

    let key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e);
    let decoded_token = jsonwebtoken::decode::<Claims>(token, &key, &validation);
    println!("decoded_token: {:?}", decoded_token);
    decoded_token
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct JWK {
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
}

#[derive(Debug, Deserialize)]
struct KeysResponse {
    keys: Vec<JWK>,
}

pub async fn get_firebase_jwks(firebase_config: &FirebaseConfig) -> Result<HashMap<String, JWK>, reqwest::Error> {
    let mut key_map = HashMap::new();
    let url =
        "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";
    let resp = reqwest::get(url).await?.json::<KeysResponse>().await?;
    for key in resp.keys {
        key_map.insert(key.kid.clone(), key);
    }

    println!("client mail: {}", &firebase_config.client_email);

    let url_client_email = format!("https://www.googleapis.com/service_accounts/v1/jwk/{}", &firebase_config.client_email);
    let resp_client_email = reqwest::get(&url_client_email).await?.json::<KeysResponse>().await?;
    for key in resp_client_email.keys {
        key_map.insert(key.kid.clone(), key);
    }

    Ok(key_map)
}
