use actix_web::HttpResponse;

#[tracing::instrument]
pub async fn health_check() -> HttpResponse {
    tracing::info!("health check kicked.");
    HttpResponse::Ok().finish()
}
