use actix_web::HttpResponse;

#[allow(clippy::async_yields_async)]
#[tracing::instrument]
pub async fn health_check() -> HttpResponse {
    tracing::info!("health check kicked.");
    HttpResponse::Ok().finish()
}
