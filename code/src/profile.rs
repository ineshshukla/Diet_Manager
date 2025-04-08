use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ActivityLevel {
    Sedentary,         // Little or no exercise
    LightlyActive,     // Light exercise/sports 1-3 days/week
    ModeratelyActive,  // Moderate exercise/sports 3-5 days/week
    VeryActive,        // Hard exercise/sports 6-7 days/week
    ExtremelyActive    // Very hard exercise/sports & physical job
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub gender: Gender,
    pub age: u32,
    pub height_cm: f32,
    pub weight_kg: f32,
    pub activity_level: ActivityLevel,
    pub target_formula: TargetFormula,
    pub daily_overrides: HashMap<String, f32>, // Date -> calorie target override
    file_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TargetFormula {
    MifflinStJeor,
    HarrisBenedict
}

impl Profile {
    pub fn new(file_path: &str) -> Self {
        Profile {
            gender: Gender::Male,
            age: 30,
            height_cm: 170.0,
            weight_kg: 70.0,
            activity_level: ActivityLevel::ModeratelyActive,
            target_formula: TargetFormula::MifflinStJeor,
            daily_overrides: HashMap::new(),
            file_path: file_path.to_string(),
        }
    }

    pub fn load(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.file_path) {
            if let Ok(loaded_profile) = serde_json::from_str(&data) {
                *self = loaded_profile;
            }
        }
    }

    pub fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(&self) {
            let _ = fs::write(&self.file_path, data);
        }
    }

    pub fn calculate_target_calories(&self) -> f32 {
        let bmr = match self.target_formula {
            TargetFormula::MifflinStJeor => self.calculate_mifflin_st_jeor(),
            TargetFormula::HarrisBenedict => self.calculate_harris_benedict(),
        };

        // Apply activity multiplier
        bmr * match self.activity_level {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::LightlyActive => 1.375,
            ActivityLevel::ModeratelyActive => 1.55,
            ActivityLevel::VeryActive => 1.725,
            ActivityLevel::ExtremelyActive => 1.9,
        }
    }

    pub fn get_daily_target(&self, date: &str) -> f32 {
        // Check if there's a custom override for this date
        if let Some(target) = self.daily_overrides.get(date) {
            *target
        } else {
            // Otherwise, calculate the target based on profile data
            self.calculate_target_calories()
        }
    }

    pub fn set_daily_override(&mut self, date: &str, target: f32) {
        self.daily_overrides.insert(date.to_string(), target);
    }

    pub fn remove_daily_override(&mut self, date: &str) {
        self.daily_overrides.remove(date);
    }

    fn calculate_mifflin_st_jeor(&self) -> f32 {
        let s = match self.gender {
            Gender::Male => 5.0,
            Gender::Female => -161.0,
        };
        (10.0 * self.weight_kg) + (6.25 * self.height_cm) - (5.0 * self.age as f32) + s
    }

    fn calculate_harris_benedict(&self) -> f32 {
        match self.gender {
            Gender::Male => {
                66.47 + (13.75 * self.weight_kg) + (5.003 * self.height_cm) - (6.755 * self.age as f32)
            },
            Gender::Female => {
                655.1 + (9.563 * self.weight_kg) + (1.850 * self.height_cm) - (4.676 * self.age as f32)
            }
        }
    }
}