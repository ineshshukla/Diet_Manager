# Diet Manager Program

## Introduction

The Diet Manager Program is a comprehensive tool designed to help users track their dietary habits, manage food entries, and monitor calorie consumption. It provides a user-friendly interface for logging meals, setting calorie targets, and viewing daily summaries. Built with Rust, this program ensures high performance and reliability.

## Steps to Use the Program

### 1. **Setup**
   - Ensure you have Rust installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/).
   - Clone this repository to your local machine.
   - Navigate to the project directory using your terminal.
   - Run `cargo build` to compile the program. This will generate the executable files.

### 2. **Running the Program**
   - Execute the program using `cargo run`.
   - Follow the on-screen instructions to interact with the program.

### 3. **Features**

#### Food Management
   - **Add Basic Food**: Add a food item with a unique identifier, keywords, and calorie information.
   - **Add Composite Food**: Create a composite food by combining multiple basic foods.
   - **List Foods**: View all foods in the database, including their calorie information.
   - **Search Foods**: Search for foods using keywords to quickly find items.

#### Daily Logging
   - **View Daily Log**: Display logged food entries for the current date, including calorie details.
   - **Log Food**: Add a food entry to the daily log with a timestamp.
   - **Remove Log Entry**: Remove a specific food entry from the daily log if needed.

#### Profile Management
   - **Edit Profile**: Update user profile details such as age, weight, height, gender, and activity level.
   - **Set Daily Target**: Set or remove a custom daily calorie target to align with your dietary goals.

#### Summary and Navigation
   - **View Daily Summary**: View a summary of calories consumed versus the target for the day.
   - **Navigation**: Use options to navigate between dates or undo the last action for flexibility.

#### Data Management
   - **Saving Data**: Save all changes using the `Save` option before exiting to ensure no data is lost.
   - **Exiting**: Use the `Exit` option to close the program safely.

## Example Usage

1. Start the program using `cargo run`.
2. Add a basic food item (e.g., "Apple") with its calorie information.
3. Log the food item to your daily log.
4. View the daily summary to check your calorie intake.
5. Save your progress before exiting.

## Additional Notes

- The program uses the Mifflin-St Jeor Equation to calculate calorie targets based on your profile.
- All data is stored locally in JSON files for easy access and modification.
- Ensure you back up your data files regularly to prevent accidental loss.

