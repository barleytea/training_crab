use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::prelude::*;

#[derive(Debug, Model, Serialize, Deserialize)]
pub struct TrainingMaster {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The name of the training menu
    #[model(index(index = "dsc", unique = "true"))]
    pub name: String,
    /// description
    pub descriptions: String,
}
