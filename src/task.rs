#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, id: usize) {
        self.tasks.retain(|task| task.id != id);
    }

    pub fn get_task(&self, id: usize) -> Option<&Task> {
       for task in &self.tasks {
           if task.id == id {
               return Some(task);
           }
       }
        None
    }

    pub fn update_task(&mut self, id: usize, utask: Task) {
      for task in &mut self.tasks.iter_mut() {
         if task.id == id {
             task.description = utask.description;
             task.completed = utask.completed;

             break;
         }
      }
    }
}