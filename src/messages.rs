use serde::{Deserialize, Serialize};

use crate::recipe::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum ListMode {
    ALL,
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRequest {
    pub mode: ListMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    pub mode: ListMode,
    pub data: Recipes,
    pub receiver: String,
}
