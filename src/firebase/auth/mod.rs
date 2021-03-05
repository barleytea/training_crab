use crate::configuration::FirebaseConfig;
use crate::firebase;
use actix_web::web;
use std::sync::Arc;

pub async fn verify_id_token(
    token: &str,
    firebase_config: web::Data<Arc<FirebaseConfig>>,
) -> Result<jsonwebtoken::TokenData<firebase::admin::jwt::Claims>, jsonwebtoken::errors::Error> {
    firebase::admin::jwt::verify(token, &firebase_config).await
}
