mod food;
mod database;

use std::io::{self, Write};
use database::Database;
use food::{Food, BasicFood, CompositeFood, compute_calories};

fn main() {
    let db_file = "food_db.json";
    let mut db = Database::new(db_file);
    db.load();
    println!("--------------Diet Manager (YADA) 🧑‍⚕️🥡🏋️‍♂️--------------\n");
    
    loop {
        println!("--------------------------------------------------------------------------------------------");
        print!("\nEnter command (1️⃣  add_basic, 2️⃣  add_composite, 3️⃣  list, 4️⃣  search, 5️⃣  save, 6️⃣  exit): ");
        io::stdout().flush().unwrap();
    
        let mut command = String::new(); 
        io::stdin().read_line(&mut command).unwrap();
    
        let command = command.trim().parse::<u32>().unwrap_or(0);
        
        match command{
            1 => {
                let mut id = String::new();
                print!("Enter basic food identifier: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id).unwrap();
                
                let mut keywords = String::new();
                print!("Enter keywords (comma separated): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut keywords).unwrap();

                let mut keywords_vec: Vec<String> = keywords
                .trim()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()) // Remove empty entries
                .collect();
        
                if !keywords_vec.contains(&id) { 
                    keywords_vec.push(id.clone()); // Ensure ID is added as a keyword
                }
                
                let mut cal_str = String::new();
                print!("Enter calories per serving: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut cal_str).unwrap();
                
                let calories: f32 = cal_str.trim().parse().unwrap_or(0.0);
                let basic = BasicFood {
                    id: id.trim().to_string(),
                    keywords: keywords_vec,
                    calories,
                };
                db.add_food(Food::Basic(basic));
                println!("Basic food added!✔️");
            },
            2 => {
                let mut id = String::new();
                print!("Enter composite food identifier: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id).unwrap();
                
                let mut keywords = String::new();
                print!("Enter keywords (comma separated): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut keywords).unwrap();
                let mut keywords_vec: Vec<String> = keywords
                .trim()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()) // Remove empty entries
                .collect();
        
                if !keywords_vec.contains(&id) { 
                    keywords_vec.push(id.clone()); // Ensure ID is added as a keyword
                }

                let mut components = Vec::new();
                print!("Enter number of components: ");
                io::stdout().flush().unwrap();
                let mut num_str = String::new();
                io::stdin().read_line(&mut num_str).unwrap();
                let num: usize = num_str.trim().parse().unwrap_or(0);
                
                for _ in 0..num {
                    let mut comp_id = String::new();
                    print!("Enter component food id: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut comp_id).unwrap();
                    
                    let mut servings_str = String::new();
                    print!("Enter number of servings: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut servings_str).unwrap();
                    let servings: f32 = servings_str.trim().parse().unwrap_or(1.0);
                    
                    if db.foods.contains_key(comp_id.trim()) {
                        components.push((comp_id.trim().to_string(), servings));
                    } else {
                        println!("Component with id '{}' not found. Skipping.", comp_id.trim());
                    }
                }
                let composite = CompositeFood {
                    id: id.trim().to_string(),
                    keywords: keywords_vec,
                    components,
                };
                db.add_food(Food::Composite(composite));
                println!("Composite food added!");
            },
            3 => {
                for food in db.foods.values() {
                    match food {
                        Food::Basic(b) => {
                            println!("Basic Food: {} | Calories: {}", b.id, b.calories);
                        },
                        Food::Composite(c) => {
                            let total_cal = compute_calories(food, &db.foods);
                            println!("Composite Food: {} | Calories (computed): {}", c.id, total_cal);
                        }
                    }
                }
            },
            4 => {
                let mut keyword = String::new();
                print!("Enter keyword to search: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut keyword).unwrap();
                let results = db.search_by_keyword(keyword.trim());
                for food in &results {
                    match food {
                        Food::Basic(b) => {
                            println!("Basic Food: {} | Calories: {}", b.id, b.calories);
                        },
                        Food::Composite(c) => {
                            let total_cal = compute_calories(food, &db.foods);
                            println!("Composite Food: {} | Calories (computed): {}", c.id, total_cal);
                        }
                    }
                }
                if results.is_empty() {
                    println!("No food found with the provided keyword.");
                }
            },
            5 => {
                db.save();
                println!("Database saved.");
            },
            6 => {
                db.save();
                println!("Database saved. Exiting.");
                break;
            },
            _ => {
                println!("Unknown command.");
            }
        }
    }
}
