use actix_web::HttpResponse;

#[allow(clippy::async_yields_async)]
#[tracing::instrument]
pub async fn health_check() -> HttpResponse {
    tracing::info!("health check kicked.");
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;

    #[actix_rt::test]
    async fn text_health_check_ok() {
        let resp = health_check().await;
        assert_eq!(resp.status(), http::StatusCode::OK)
    }
}
