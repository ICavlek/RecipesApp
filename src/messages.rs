use serde::{Deserialize, Serialize};

use crate::recipe::Recipe;
type Recipes = Vec<Recipe>;

#[derive(Debug, Serialize, Deserialize)]
pub enum ListMode {
    ALL,
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    pub mode: ListMode,
    pub data: Recipes,
    pub receiver: String,
}

pub enum EventType {
    Response(ListResponse),
    Input(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRequest {
    pub mode: ListMode,
}
