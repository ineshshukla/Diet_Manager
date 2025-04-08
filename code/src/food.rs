use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicFood {
    pub id: String,
    pub keywords: Vec<String>,
    pub calories: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompositeFood {
    pub id: String,
    pub keywords: Vec<String>,
    pub components: Vec<(String, f32)>, // (component food id, servings)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Food {
    Basic(BasicFood),
    Composite(CompositeFood),
}

// Recursively computes the calories for a food item given the entire foods database.
pub fn compute_calories(food: &Food, food_map: &HashMap<String, Food>) -> f32 {
    match food {
        Food::Basic(basic) => basic.calories,
        Food::Composite(composite) => {
            composite.components.iter().fold(0.0, |sum, (id, servings)| {
                if let Some(component_food) = food_map.get(id) {
                    sum + compute_calories(component_food, food_map) * servings
                } else {
                    sum
                }
            })
        }
    }
}
