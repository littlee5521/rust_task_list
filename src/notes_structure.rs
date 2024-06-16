#[derive(Debug)]
pub struct Task {
    pub task_body: String,
    pub completed: bool,
}

impl Task {
    pub fn switch_task_state(&mut self) {
        self.completed = !self.completed;
    }
    pub fn edit_task_body(&mut self, text: String) {
        self.task_body = text
    }
}
pub struct TaskGroup {
    pub title: String,
    pub completed: bool,
    pub manager: Vec<Task>,
}
impl TaskGroup {
    pub fn change_title(&mut self, new_title: String) {
        self.title = new_title
    }
    pub fn add_task(&mut self, task: Task) {
        self.manager.push(task)
    }
    pub fn print_task_list(&self) {
        for (index, task) in self.manager.iter().enumerate() {
            let response = format!(
                "{}: {}, Completed: {}",
                index, task.task_body, task.completed
            );
            println!("{}", response)
        }
    }
    pub fn remove_task(&mut self, index: usize) {
        self.manager.remove(index);
    }
}

pub struct TaskManager {
    pub group: Vec<TaskGroup>,
}
impl TaskManager {
    pub fn print_group_list(&self) {
        for (index, group) in self.group.iter().enumerate() {
            let response = format!("{}: {}, Completed: {}", index, group.title, group.completed);
            println!("{}", response)
        }
    }
}
