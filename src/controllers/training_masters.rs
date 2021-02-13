use crate::models::training_masters::TrainingMaster;
use actix_web::{web, HttpResponse};
use handlebars::to_json;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use wither::bson::{doc, oid::ObjectId};
use wither::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingMasterData {
    pub name: String,
    pub descriptions: String,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument]
pub async fn create_training_master(
    item: web::Json<TrainingMasterData>,
    database: web::Data<Arc<Database>>,
) -> HttpResponse {
    let mut training_master: TrainingMaster = TrainingMaster {
        id: None,
        name: (&item.name).parse().unwrap(),
        descriptions: (&item.descriptions).parse().unwrap(),
    };

    let saved_training_master: wither::Result<()> = training_master.save(&database, None).await;
    match saved_training_master {
        Ok(_) => HttpResponse::Created().json(&item.0),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument]
pub async fn find_training_master(
    query: web::Query<HashMap<String, String>>,
    database: web::Data<Arc<Database>>,
) -> HttpResponse {
    let training_master_id_maybe = query.get("training_master_id");
    if training_master_id_maybe.is_none() {
        return HttpResponse::BadRequest().finish();
    }
    let object_id = ObjectId::with_string(training_master_id_maybe.unwrap()).unwrap();
    let filter = doc! { "_id": object_id };
    let training_master: wither::Result<Option<TrainingMaster>> =
        TrainingMaster::find_one(&database, filter, None).await;
    match training_master {
        Ok(t) => HttpResponse::Ok().json(to_json(&t)),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}
