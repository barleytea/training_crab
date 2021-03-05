use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::prelude::*;

#[derive(Debug, Model, Serialize, Deserialize)]
pub struct User {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The ID of firebase userid
    #[model(index(index = "dsc"))]
    pub uid: String,
    /// The name of the user
    pub nick_name: String,
}
