use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId, DateTime};
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
    /// last logged in
    pub last_logged_in_at: DateTime,
    /// created
    pub created_at: DateTime,
    /// updated
    pub updated_at: DateTime,
}
