use crate::models::users::User;
use actix_web::{web, HttpResponse};
use handlebars::to_json;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use wither::bson::doc;
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

#[allow(clippy::async_yields_async)]
#[tracing::instrument]
pub async fn find_user(
    query: web::Query<HashMap<String, String>>,
    database: web::Data<Arc<Database>>,
) -> HttpResponse {
    let uid_maybe = query.get("uid");
    if uid_maybe.is_none() {
        return HttpResponse::BadRequest().finish();
    }
    let uid = uid_maybe.unwrap();
    let filter = doc! { "uid": uid };
    let user: wither::Result<Option<User>> = User::find_one(&database, filter, None).await;
    match user {
        Ok(u) => HttpResponse::Ok().json(to_json(&u)),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}
