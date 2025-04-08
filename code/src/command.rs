use crate::database::Database;
use crate::food::{Food, BasicFood, CompositeFood};
use crate::log::DailyLog;
use std::fmt;

// Trait defining an undoable command
pub trait UndoableCommand: fmt::Debug {
    fn execute(&mut self) -> bool;
    fn undo(&mut self) -> bool;
    fn description(&self) -> String;
}

// Command for adding a food to the database
#[derive(Debug)]
pub struct AddFoodCommand {
    food: Food,
    db: *mut Database,  // Using raw pointers to avoid ownership issues
}

impl AddFoodCommand {
    pub fn new(food: Food, db: &mut Database) -> Self {
        AddFoodCommand {
            food,
            db: db as *mut Database,
        }
    }
}

impl UndoableCommand for AddFoodCommand {
    fn execute(&mut self) -> bool {
        unsafe {
            let db = &mut *self.db;
            let id = match &self.food {
                Food::Basic(basic) => basic.id.clone(),
                Food::Composite(composite) => composite.id.clone(),
            };
            db.add_food(self.food.clone());
            true
        }
    }

    fn undo(&mut self) -> bool {
        unsafe {
            let db = &mut *self.db;
            let id = match &self.food {
                Food::Basic(basic) => basic.id.clone(),
                Food::Composite(composite) => composite.id.clone(),
            };
            db.foods.remove(&id);
            true
        }
    }

    fn description(&self) -> String {
        let id = match &self.food {
            Food::Basic(basic) => format!("basic food '{}'", basic.id),
            Food::Composite(composite) => format!("composite food '{}'", composite.id),
        };
        format!("Add {}", id)
    }
}

// Command for logging food in daily log
#[derive(Debug)]
pub struct LogFoodCommand {
    date: String,
    food_id: String,
    servings: f32,
    log: *mut DailyLog,
    entry_index: Option<usize>, // Store the index for undoing
}

impl LogFoodCommand {
    pub fn new(date: &str, food_id: &str, servings: f32, log: &mut DailyLog) -> Self {
        LogFoodCommand {
            date: date.to_string(),
            food_id: food_id.to_string(),
            servings,
            log: log as *mut DailyLog,
            entry_index: None,
        }
    }
}

impl UndoableCommand for LogFoodCommand {
    fn execute(&mut self) -> bool {
        unsafe {
            let log = &mut *self.log;
            
            // Get the current number of entries for this date to determine the index
            let entries = log.get_log_entries(&self.date);
            self.entry_index = Some(entries.len());
            
            log.add_food(&self.date, &self.food_id, self.servings);
            true
        }
    }

    fn undo(&mut self) -> bool {
        unsafe {
            let log = &mut *self.log;
            if let Some(index) = self.entry_index {
                return log.remove_food(&self.date, index);
            }
            false
        }
    }

    fn description(&self) -> String {
        format!("Log {:.1} serving(s) of '{}' on {}", self.servings, self.food_id, self.date)
    }
}

// Command for removing food from daily log
#[derive(Debug)]
pub struct RemoveLogEntryCommand {
    date: String,
    index: usize,
    food_id: String,
    servings: f32,
    log: *mut DailyLog,
}

impl RemoveLogEntryCommand {
    pub fn new(date: &str, index: usize, food_id: &str, servings: f32, log: &mut DailyLog) -> Self {
        RemoveLogEntryCommand {
            date: date.to_string(),
            index,
            food_id: food_id.to_string(),
            servings,
            log: log as *mut DailyLog,
        }
    }
}

impl UndoableCommand for RemoveLogEntryCommand {
    fn execute(&mut self) -> bool {
        unsafe {
            let log = &mut *self.log;
            log.remove_food(&self.date, self.index)
        }
    }

    fn undo(&mut self) -> bool {
        unsafe {
            let log = &mut *self.log;
            log.add_food(&self.date, &self.food_id, self.servings);
            true
        }
    }

    fn description(&self) -> String {
        format!("Remove entry #{} (food: '{}') from {}", self.index + 1, self.food_id, self.date)
    }
}

// Main CommandManager to handle the undo stack
#[derive(Debug)]
pub struct CommandManager {
    undo_stack: Vec<Box<dyn UndoableCommand>>,
}

impl CommandManager {
    pub fn new() -> Self {
        CommandManager {
            undo_stack: Vec::new(),
        }
    }

    pub fn execute_command(&mut self, mut command: Box<dyn UndoableCommand>) -> bool {
        let success = command.execute();
        if success {
            self.undo_stack.push(command);
        }
        success
    }

    pub fn undo_last_command(&mut self) -> Option<String> {
        if let Some(mut command) = self.undo_stack.pop() {
            let description = command.description();
            match command.undo() {
                true => Some(description),
                false => None,
            }
        } else {
            None
        }
    }

    pub fn has_commands(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn command_history(&self) -> Vec<String> {
        self.undo_stack.iter()
            .map(|cmd| cmd.description())
            .collect()
    }
}