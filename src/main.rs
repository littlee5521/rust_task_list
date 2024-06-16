mod notes_structure;
use notes_structure::{Task, TaskGroup, TaskManager};
use std::io;

fn main() {
    //Iniializes the main task structure
    let mut main_task = TaskManager { group: vec![] };

    println!("Hello, welcome to the task manager");
    //this will be the main app loop. Which will go into a nested loop for handling subgroups of task
    loop {
        main_task.print_group_list();
        println!(
            "Press 0 to start a new task group. Press 1 to delete a task group. Press 2 to quit"
        );
        //this recives the users choice
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");
        let user_choice: u32 = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                print!("{}", e);
                continue;
            }
        };
        match user_choice {
            0 => {}
            1 => {}
            2 => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
}
