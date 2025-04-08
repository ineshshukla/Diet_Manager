mod food;
mod database;
mod log;
mod command;
mod profile;

use std::io::{self, Write};
use database::Database;
use food::{Food, BasicFood, CompositeFood, compute_calories};
use log::{DailyLog, LogEntry};
use chrono::{Local, NaiveDate, Duration};
use command::{CommandManager, UndoableCommand, AddFoodCommand, LogFoodCommand, RemoveLogEntryCommand};
use profile::{Profile, Gender, ActivityLevel, TargetFormula};
use colored::*;

struct AppState {
    current_date: String,
    db: Database,
    daily_log: DailyLog,
    command_manager: CommandManager,
    profile: Profile,
}

fn main() {
    let db_file = "food_db.json";
    let log_file = "log.json";
    let profile_file = "profile.json";
    
    let mut state = AppState {
        current_date: Local::now().naive_local().date().format("%Y-%m-%d").to_string(),
        db: Database::new(db_file),
        daily_log: DailyLog::new(log_file),
        command_manager: CommandManager::new(),
        profile: Profile::new(profile_file),
    };
    
    state.db.load();
    state.daily_log.load();
    state.profile.load();
    println!("{}", "-------------- Diet Manager (YADA) 🧑‍⚕️🥡🏋️‍♂️ --------------".bold().underline().blue());
    
    loop {
        println!("{}", "--------------------------------------------------------------------------------------------".bright_black());
        println!("📆 {}: {}", "Current Date".bold(), state.current_date.bright_cyan().bold());
        println!("{}", "--------------------------------------------------------------------------------------------".bright_black());
        println!("{}", "Main Menu".bold().underline().bright_yellow());
        println!("\n{}", "Food Database:".bold().bright_magenta());
        println!("  {} Add Basic Food    {} Add Composite Food    {} List Foods    {} Search Foods",
                 "1".bold().bright_green(), "2".bold().bright_green(), "3".bold().bright_green(), "4".bold().bright_green());
        println!("\n{} Daily Log ({}):", "Daily Log:".bold().bright_magenta(), state.current_date);
        println!("  {} View Log          {} Log Food              {} Remove Log Entry",
                 "5".bold().bright_green(), "6".bold().bright_green(), "7".bold().bright_green());
        println!("\n{} Profile & Targets:", "Profile & Targets:".bold().bright_magenta());
        println!("  {} Edit Profile      {} Set Daily Target      {} View Daily Summary",
                 "8".bold().bright_green(), "9".bold().bright_green(), "10".bold().bright_green());
        println!("\n{} Date Navigation:", "Date Navigation:".bold().bright_magenta());
        println!("  {} Select Date       {} Previous Day          {} Next Day",
                 "11".bold().bright_green(), "12".bold().bright_green(), "13".bold().bright_green());
        println!("\n{} System:", "System:".bold().bright_magenta());
        println!("  {} Save              {} Exit                  {} Undo Last Action",
                 "14".bold().bright_green(), "15".bold().bright_green(), "16".bold().bright_green());
        
        if state.command_manager.has_commands() {
            println!("{}", "↩️ Undo available. You can revert the last action.".italic().bright_green());
        }
        
        print!("\n{}: ", "Enter Choice".bold().bright_yellow());
        io::stdout().flush().unwrap();
    
        let mut command = String::new(); 
        io::stdin().read_line(&mut command).unwrap();
    
        let command = command.trim().parse::<u32>().unwrap_or(0);
        
        match command {
            1 => add_basic_food(&mut state),
            2 => add_composite_food(&mut state),
            3 => list_foods(&state),
            4 => search_foods(&state),
            5 => view_daily_log(&state),
            6 => log_food_entry(&mut state),
            7 => remove_log_entry(&mut state),
            8 => edit_profile(&mut state),
            9 => set_daily_target(&mut state),
            10 => view_daily_summary(&state),
            11 => select_date(&mut state),
            12 => {
                if let Ok(date) = NaiveDate::parse_from_str(&state.current_date, "%Y-%m-%d") {
                    let prev_day = date - Duration::days(1);
                    state.current_date = prev_day.format("%Y-%m-%d").to_string();
                    println!("📆 {}: {}", "Date Changed To".bold(), state.current_date.bright_cyan());
                }
            },
            13 => {
                if let Ok(date) = NaiveDate::parse_from_str(&state.current_date, "%Y-%m-%d") {
                    let next_day = date + Duration::days(1);
                    state.current_date = next_day.format("%Y-%m-%d").to_string();
                    println!("📆 {}: {}", "Date Changed To".bold(), state.current_date.bright_cyan());
                }
            },
            14 => {
                state.db.save();
                state.daily_log.save();
                state.profile.save();
                println!("{}", "💾 All data saved! ✅".green().bold());
            },
            15 => {
                state.db.save();
                state.daily_log.save();
                state.profile.save();
                println!("{}", "📁 Database and logs saved. Exiting. 👋".cyan());
                break;
            },
            16 => {
                if let Some(action) = state.command_manager.undo_last_command() {
                    println!("{} {}", "✅ Undid Action:".green().bold(), action);
                    state.db.save();
                    state.daily_log.save();
                } else {
                    println!("{}", "❌ Nothing to undo.".red().bold());
                }
            },
            _ => {
                println!("{}", "❌ Unknown Command. Please enter a valid option.".red().bold());
            }
        }

        println!("\nPress Enter to continue...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}

fn add_basic_food(state: &mut AppState) {
    let mut id = String::new();
    print!("{}", "Enter basic food identifier: 🥗 ".bright_yellow());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).unwrap();
    id = id.trim().to_string();
    
    let mut keywords = String::new();
    print!("{}", "Enter keywords (comma separated): 🔍 ".magenta());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut keywords).unwrap();

    let mut keywords_vec: Vec<String> = keywords
    .trim()
    .split(',')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect();

    if !keywords_vec.contains(&id) { 
        keywords_vec.push(id.clone());
    }
    
    let mut cal_str = String::new();
    print!("Enter calories per serving: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cal_str).unwrap();
    
    let calories: f32 = cal_str.trim().parse().unwrap_or(0.0);
    let basic = BasicFood {
        id: id.clone(),
        keywords: keywords_vec,
        calories,
    };
    
    let food = Food::Basic(basic);
    let command = Box::new(AddFoodCommand::new(food, &mut state.db));
    
    if state.command_manager.execute_command(command) {
        println!("{}", "✅ Composite food added successfully!".green().bold());
    } else {
        println!("{}", "❌ Failed to add food. Please try again.".red().bold());
    }
}

fn add_composite_food(state: &mut AppState) {
    let mut id = String::new();
    print!("{}", "Enter composite food identifier: 🥗 ".bright_yellow());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).unwrap();
    id = id.trim().to_string();
    
    let mut keywords = String::new();
    print!("{}", "Enter keywords (comma separated): 🔍 ".magenta());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut keywords).unwrap();
    let mut keywords_vec: Vec<String> = keywords
    .trim()
    .split(',')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect();

    if !keywords_vec.contains(&id) { 
        keywords_vec.push(id.clone());
    }

    let mut components = Vec::new();
    print!("{}", "Enter number of components: 🧩 ".bright_yellow());
    io::stdout().flush().unwrap();
    let mut num_str = String::new();
    io::stdin().read_line(&mut num_str).unwrap();
    let num: usize = num_str.trim().parse().unwrap_or(0);
    
    for _ in 0..num {
        let comp_id = select_food_component(&state.db);
        
        if comp_id.is_empty() {
            println!("Component selection cancelled.");
            continue;
        }
        
        let mut servings_str = String::new();
        print!("Enter number of servings: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut servings_str).unwrap();
        let servings: f32 = servings_str.trim().parse().unwrap_or(1.0);
        
        components.push((comp_id, servings));
    }
    
    let composite = CompositeFood {
        id: id.clone(),
        keywords: keywords_vec,
        components,
    };
    
    let food = Food::Composite(composite);
    let command = Box::new(AddFoodCommand::new(food, &mut state.db));
    
    if state.command_manager.execute_command(command) {
        println!("{}", "✅ Composite food added successfully!".green().bold());
    } else {
        println!("{}", "❌ Failed to add food. Please try again.".red().bold());
    }
}

fn select_food_component(db: &Database) -> String {
    loop {
        println!("\nSelect component food:");
        println!("1. Search by keyword");
        println!("2. List all foods");
        println!("3. Cancel");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);
        
        match choice {
            1 => {
                let mut keyword = String::new();
                print!("Enter keyword to search: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut keyword).unwrap();
                let results = db.search_by_keyword(keyword.trim());
                
                if results.is_empty() {
                    println!("{}", "No foods found with that keyword. ❌".red());
                    continue;
                }
                
                return display_food_selection(results);
            },
            2 => {
                let all_foods: Vec<&Food> = db.foods.values().collect();
                if all_foods.is_empty() {
                    println!("No foods in database.");
                    continue;
                }
                
                return display_food_selection(all_foods);
            },
            3 => return String::new(),
            _ => println!("Invalid choice."),
        }
    }
}

fn display_food_selection(foods: Vec<&Food>) -> String {
    for (i, food) in foods.iter().enumerate() {
        match food {
            Food::Basic(b) => {
                println!("{}. Basic: {} - {:.1} calories/serving", i+1, b.id, b.calories);
            },
            Food::Composite(c) => {
                println!("{}. Composite: {}", i+1, c.id);
            }
        }
    }
    
    print!("{}", "Select food number (or 0 to cancel): 🔢 ".bright_green());
    io::stdout().flush().unwrap();
    let mut selection = String::new();
    io::stdin().read_line(&mut selection).unwrap();
    let selection: usize = selection.trim().parse().unwrap_or(0);
    
    if selection == 0 || selection > foods.len() {
        return String::new();
    }
    
    match foods[selection-1] {
        Food::Basic(b) => b.id.clone(),
        Food::Composite(c) => c.id.clone()
    }
}

fn list_foods(state: &AppState) {
    if state.db.foods.is_empty() {
        println!("No foods in database.");
        return;
    }
    
    println!("Foods in database:");
    for food in state.db.foods.values() {
        match food {
            Food::Basic(b) => {
                println!("Basic Food: {} | Calories: {:.1}", b.id, b.calories);
            },
            Food::Composite(c) => {
                let total_cal = compute_calories(food, &state.db.foods);
                println!("Composite Food: {} | Calories (computed): {:.1}", c.id, total_cal);
            }
        }
    }
}

fn search_foods(state: &AppState) {
    let mut keyword = String::new();
    print!("Enter keyword to search: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut keyword).unwrap();
    let results = state.db.search_by_keyword(keyword.trim());
    
    if results.is_empty() {
        println!("No food found with the provided keyword.");
        return;
    }
    
    println!("Search results:");
    for food in &results {
        match food {
            Food::Basic(b) => {
                println!("Basic Food: {} | Calories: {:.1}", b.id, b.calories);
            },
            Food::Composite(c) => {
                let total_cal = compute_calories(food, &state.db.foods);
                println!("Composite Food: {} | Calories (computed): {:.1}", c.id, total_cal);
            }
        }
    }
}

fn view_daily_log(state: &AppState) {
    if !state.daily_log.has_entries_for_date(&state.current_date) {
        println!("No food entries for {} 📅", state.current_date);
        return;
    }
    
    println!("🍽️ Food log for {}: 📅", state.current_date);
    print_daily_log_entries(state);
    
    let total_calories = state.daily_log.get_total_calories(&state.current_date, &state.db.foods);
    println!("{}", "📊 Total calories for the day: 🔥".bold().yellow());
}

fn print_daily_log_entries(state: &AppState) {
    let entries = state.daily_log.get_log_entries(&state.current_date);
    
    for (i, entry) in entries.iter().enumerate() {
        let food_name = &entry.food_id;
        let calories = match state.db.foods.get(food_name) {
            Some(food) => compute_calories(food, &state.db.foods) * entry.servings,
            None => 0.0,
        };
        println!("{}. {} - {:.1} serving(s), {:.1} calories", 
                 i+1, food_name, entry.servings, calories);
    }
}

fn log_food_entry(state: &mut AppState) {
    println!("\nAdd food to log for {}: 📝", state.current_date);
    println!("1. Search by keyword");
    println!("2. List all foods");
    println!("3. Cancel");
    print!("Enter choice: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse().unwrap_or(0);
    
    let food_id = match choice {
        1 => {
            // Prompt for comma-separated keywords
            let mut keywords_input = String::new();
            print!("{}", "Enter keywords (comma separated): 🔍 ".magenta());
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut keywords_input).unwrap();
            let keywords: Vec<String> = keywords_input
                .trim()
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect();
            if keywords.is_empty() {
                println!("No keywords entered.");
                return;
            }
            // Prompt user to choose matching mode
            print!("{}", "Match all keywords? (y/n): ".bright_yellow());
            io::stdout().flush().unwrap();
            let mut mode_input = String::new();
            io::stdin().read_line(&mut mode_input).unwrap();
            let match_all = mode_input.trim().eq_ignore_ascii_case("y");
            
            // Manual filtering of foods based on keywords and matching mode
            let results: Vec<&Food> = state.db.foods.values().filter(|food| {
                let food_keywords: Vec<String> = match food {
                    Food::Basic(b) => b.keywords.iter().map(|s| s.to_lowercase()).collect(),
                    Food::Composite(c) => c.keywords.iter().map(|s| s.to_lowercase()).collect(),
                };
                if match_all {
                    keywords.iter().all(|kw| food_keywords.contains(kw))
                } else {
                    keywords.iter().any(|kw| food_keywords.contains(kw))
                }
            }).collect();
            
            if results.is_empty() {
                println!("No foods found with the provided keywords.");
                return;
            }
            display_food_selection(results)
        },
        2 => {
            let all_foods: Vec<&Food> = state.db.foods.values().collect();
            if all_foods.is_empty() {
                println!("No foods in database.");
                return;
            }
            display_food_selection(all_foods)
        },
        _ => return,
    };
    
    if food_id.is_empty() {
        println!("Food selection cancelled.");
        return;
    }
    
    let mut servings_str = String::new();
    print!("Enter number of servings: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut servings_str).unwrap();
    let servings: f32 = servings_str.trim().parse().unwrap_or(1.0);
    
    let command = Box::new(LogFoodCommand::new(
        &state.current_date,
        &food_id,
        servings,
        &mut state.daily_log
    ));
    
    if state.command_manager.execute_command(command) {
        let calories = match state.db.foods.get(&food_id) {
            Some(food) => compute_calories(food, &state.db.foods) * servings,
            None => 0.0,
        };
        
        println!("✅ Logged {:.1} serving(s) of {} ({:.1} calories) for {}", 
            servings, food_id, calories, state.current_date);
        state.daily_log.save();
    } else {
        println!("❌ Failed to log food.");
    }
}

fn remove_log_entry(state: &mut AppState) {
    if !state.daily_log.has_entries_for_date(&state.current_date) {
        println!("No entries found for date: {}", state.current_date);
        return;
    }
    
    println!("Entries for {}:", state.current_date);
    print_daily_log_entries(state);
    
    let mut index_str = String::new();
    print!("Enter number to remove (or 0 to cancel): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut index_str).unwrap();
    let index: usize = index_str.trim().parse().unwrap_or(0);
    
    if index == 0 {
        return;
    }
    
    let entries = state.daily_log.get_log_entries(&state.current_date);
    if index <= entries.len() {
        let entry = entries[index - 1];
        let food_id = entry.food_id.clone();
        let servings = entry.servings;
        
        let command = Box::new(RemoveLogEntryCommand::new(
            &state.current_date,
            index - 1,
            &food_id,
            servings,
            &mut state.daily_log
        ));
        
        if state.command_manager.execute_command(command) {
            println!("✅ Entry removed successfully.");
            state.daily_log.save();
        } else {
            println!("❌ Failed to remove entry.");
        }
    } else {
        println!("{}", "❌ Invalid selection. Please try again.".red());
    }
}

fn select_date(state: &mut AppState) {
    let today = Local::now().naive_local().date();
    let mut date_str = String::new();
    print!("{}", "Enter date (YYYY-MM-DD), or press enter for today: 📅 ".cyan());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut date_str).unwrap();
    date_str = date_str.trim().to_string();
    
    if date_str.is_empty() {
        state.current_date = today.format("%Y-%m-%d").to_string();
    } else {
        match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            Ok(_) => state.current_date = date_str,
            Err(_) => {
                println!("Invalid date format. Using today's date instead.");
                state.current_date = today.format("%Y-%m-%d").to_string();
            }
        }
    }
    println!("📆 Date changed to: {}", state.current_date);
}

fn edit_profile(state: &mut AppState) {
    println!("\nEdit Profile Settings:");
    println!("1. Gender ({:?})", state.profile.gender);
    println!("2. Age ({})", state.profile.age);
    println!("3. Height ({} cm)", state.profile.height_cm);
    println!("4. Weight ({} kg)", state.profile.weight_kg);
    println!("5. Activity Level ({:?})", state.profile.activity_level);
    println!("6. Target Formula ({:?})", state.profile.target_formula);
    println!("7. Cancel");
    print!("Choose setting to edit: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse().unwrap_or(0);
    
    match choice {
        1 => {
            println!("Select gender:");
            println!("1. Male");
            println!("2. Female");
            print!("Enter choice: ");
            io::stdout().flush().unwrap();
            let mut gender_choice = String::new();
            io::stdin().read_line(&mut gender_choice).unwrap();
            state.profile.gender = match gender_choice.trim() {
                "2" => Gender::Female,
                _ => Gender::Male,
            };
        },
        2 => {
            print!("Enter age: ");
            io::stdout().flush().unwrap();
            let mut age = String::new();
            io::stdin().read_line(&mut age).unwrap();
            if let Ok(age) = age.trim().parse() {
                state.profile.age = age;
            }
        },
        3 => {
            print!("Enter height (cm): ");
            io::stdout().flush().unwrap();
            let mut height = String::new();
            io::stdin().read_line(&mut height).unwrap();
            if let Ok(height) = height.trim().parse() {
                state.profile.height_cm = height;
            }
        },
        4 => {
            print!("Enter weight (kg): ");
            io::stdout().flush().unwrap();
            let mut weight = String::new();
            io::stdin().read_line(&mut weight).unwrap();
            if let Ok(weight) = weight.trim().parse() {
                state.profile.weight_kg = weight;
            }
        },
        5 => {
            println!("Select activity level:");
            println!("1. Sedentary (little or no exercise)");
            println!("2. Lightly Active (light exercise 1-3 days/week)");
            println!("3. Moderately Active (moderate exercise 3-5 days/week)");
            println!("4. Very Active (hard exercise 6-7 days/week)");
            println!("5. Extremely Active (very hard exercise & physical job)");
            print!("Enter choice: ");
            io::stdout().flush().unwrap();
            let mut level = String::new();
            io::stdin().read_line(&mut level).unwrap();
            state.profile.activity_level = match level.trim() {
                "1" => ActivityLevel::Sedentary,
                "2" => ActivityLevel::LightlyActive,
                "3" => ActivityLevel::ModeratelyActive,
                "4" => ActivityLevel::VeryActive,
                "5" => ActivityLevel::ExtremelyActive,
                _ => ActivityLevel::ModeratelyActive,
            };
        },
        6 => {
            println!("Select target formula:");
            println!("1. Mifflin-St Jeor (recommended)");
            println!("2. Harris-Benedict");
            print!("Enter choice: ");
            io::stdout().flush().unwrap();
            let mut formula = String::new();
            io::stdin().read_line(&mut formula).unwrap();
            state.profile.target_formula = match formula.trim() {
                "2" => TargetFormula::HarrisBenedict,
                _ => TargetFormula::MifflinStJeor,
            };
        },
        _ => return,
    }
    
    state.profile.save();
    println!("{}", "✅ Profile updated successfully! 🎉".green().bold());
}

fn set_daily_target(state: &mut AppState) {
    let current_target = state.profile.get_daily_target(&state.current_date);
    println!("\nCurrent target for {}: {:.0} calories", state.current_date, current_target);
    println!("1. Set custom target");
    println!("2. Remove custom target (use calculated target)");
    println!("3. Cancel");
    print!("Choose option: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    match choice.trim() {
        "1" => {
            print!("Enter target calories: ");
            io::stdout().flush().unwrap();
            let mut target = String::new();
            io::stdin().read_line(&mut target).unwrap();
            if let Ok(calories) = target.trim().parse() {
                state.profile.set_daily_override(&state.current_date, calories);
                state.profile.save();
                println!("{}", "✅ Daily target set successfully! 🎯".green().bold());
            }
        },
        "2" => {
            state.profile.remove_daily_override(&state.current_date);
            state.profile.save();
            let new_target = state.profile.calculate_target_calories();
            println!("{}", "✅ Custom target removed. Using calculated target. 🎯".green().bold());
        },
        _ => return,
    }
}

fn view_daily_summary(state: &AppState) {
    let target = state.profile.get_daily_target(&state.current_date);
    let consumed = state.daily_log.get_total_calories(&state.current_date, &state.db.foods);
    let difference = consumed - target;
    
    println!("\n📊 Daily Summary for {}:", state.current_date);
    println!("{}", "🎯 Target Calories:".bold().blue());
    println!("{}", "🍽️ Consumed Calories:".bold().green());
    println!("{}", "📈 Difference:".bold().red());
    
    if state.daily_log.has_entries_for_date(&state.current_date) {
        println!("\nFood entries:");
        print_daily_log_entries(state);
    }
}
