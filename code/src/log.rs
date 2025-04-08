use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use crate::food::{Food, compute_calories};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub food_id: String,
    pub servings: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyLog {
    // Maps dates (as strings) to a vector of consumed food entries
    logs: HashMap<String, Vec<LogEntry>>,
    file_path: String,
}

impl DailyLog {
    pub fn new(file_path: &str) -> Self {
        DailyLog {
            logs: HashMap::new(),
            file_path: file_path.to_string(),
        }
    }

    pub fn load(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.file_path) {
            if let Ok(loaded_logs) = serde_json::from_str::<HashMap<String, Vec<LogEntry>>>(&data) {
                self.logs = loaded_logs;
            }
        }
    }

    pub fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(&self.logs) {
            let _ = fs::write(&self.file_path, data);
        }
    }

    pub fn add_food(&mut self, date: &str, food_id: &str, servings: f32) {
        let entry = LogEntry {
            food_id: food_id.to_string(),
            servings,
        };
        
        self.logs.entry(date.to_string())
            .or_insert_with(Vec::new)
            .push(entry);
    }
    
    pub fn remove_food(&mut self, date: &str, index: usize) -> bool {
        if let Some(entries) = self.logs.get_mut(date) {
            if index < entries.len() {
                entries.remove(index);
                return true;
            }
        }
        false
    }
    
    pub fn get_total_calories(&self, date: &str, foods: &HashMap<String, Food>) -> f32 {
        if let Some(entries) = self.logs.get(date) {
            return entries.iter().map(|entry| {
                if let Some(food) = foods.get(&entry.food_id) {
                    compute_calories(food, foods) * entry.servings
                } else {
                    0.0
                }
            }).sum();
        }
        0.0
    }
    
    pub fn get_log_entries(&self, date: &str) -> Vec<&LogEntry> {
        if let Some(entries) = self.logs.get(date) {
            entries.iter().collect()
        } else {
            vec![]
        }
    }
    
    pub fn has_entries_for_date(&self, date: &str) -> bool {
        self.logs.contains_key(date) && !self.logs.get(date).unwrap().is_empty()
    }
    
    pub fn get_all_dates(&self) -> Vec<&String> {
        self.logs.keys().collect()
    }
}