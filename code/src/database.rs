use std::collections::HashMap;
use std::fs;
use serde_json;
use crate::food::Food;

pub struct Database {
    pub foods: HashMap<String, Food>,
    pub file_path: String,
}

impl Database {
    pub fn new(file_path: &str) -> Self {
        Database {
            foods: HashMap::new(),
            file_path: file_path.to_string(),
        }
    }

    pub fn load(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.file_path) {
            if let Ok(loaded_foods) = serde_json::from_str::<Vec<Food>>(&data) {
                self.foods = loaded_foods.into_iter().map(|f| {
                    let id = match &f {
                        Food::Basic(b) => b.id.clone(),
                        Food::Composite(c) => c.id.clone(),
                    };
                    (id, f)
                }).collect();
            }
        }
    }

    pub fn save(&self) {
        let foods_vec: Vec<&Food> = self.foods.values().collect();
        if let Ok(data) = serde_json::to_string_pretty(&foods_vec) {
            let _ = fs::write(&self.file_path, data);
        }
    }

    pub fn add_food(&mut self, food: Food) {
        let id = match &food {
            Food::Basic(b) => b.id.clone(),
            Food::Composite(c) => c.id.clone(),
        };
        self.foods.insert(id, food);
    }
    
    pub fn search_by_keyword(&self, keyword: &str) -> Vec<&Food> {
        self.foods.values().filter(|food| {
            match food {
                Food::Basic(b) => b.keywords.iter().any(|kw| kw.contains(keyword)),
                Food::Composite(c) => c.keywords.iter().any(|kw| kw.contains(keyword)),
            }
        }).collect()
    }
}
