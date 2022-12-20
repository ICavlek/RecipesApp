use serde::{Deserialize, Serialize};

pub type Recipes = Vec<Recipe>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    id: usize,
    name: String,
    ingredients: String,
    instructions: String,
    public: bool,
}
