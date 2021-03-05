use actix_web::Error;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use crate::firebase::auth::verify_id_token;


#[allow(clippy::async_yields_async)]
#[tracing::instrument]
pub async fn validate(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config: Config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match verify_id_token(credentials.token()).await {
        Ok(_res) => Ok(req),
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}


