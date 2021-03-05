use crate::configuration::FirebaseConfig;
use crate::firebase::auth::verify_id_token;
use actix_web::dev::ServiceRequest;
use actix_web::{web, Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use std::sync::Arc;

#[allow(clippy::async_yields_async)]
#[tracing::instrument]
pub async fn validate(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config: Config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    let firebase_config = req
        .app_data::<web::Data<Arc<FirebaseConfig>>>()
        .map(|data| data.clone())
        .unwrap();
    match verify_id_token(credentials.token(), firebase_config).await {
        Ok(_res) => Ok(req),
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
