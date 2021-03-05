use crate::models::users::User;
use actix_web::{web, HttpResponse};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wither::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub uid: String,
    pub nick_name: String,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument]
pub async fn create_user(
    item: web::Json<UserData>,
    database: web::Data<Arc<Database>>,
) -> HttpResponse {
    let mut user: User = User {
        id: None,
        uid: (&item.uid).parse().unwrap(),
        nick_name: (&item.nick_name).parse().unwrap(),
    };

    let saved_user: wither::Result<()> = user.save(&database, None).await;
    match saved_user {
        Ok(_) => HttpResponse::Created().json(&item.0),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}
