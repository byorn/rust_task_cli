use crate::task::{Task, TaskList};

mod task;

fn main() {
    let mut task_list = TaskList::new();

    for i in 0..10 {
        let task = Task {
            id: i,
            description: "test".to_string()+i.to_string().as_str(),
            completed: false,
        };

        task_list.add_task(task);
    }

    task_list.remove_task(5);

    task_list.update_task(3, Task{
        id: 444,
        description: "temp".to_string(),
        completed: true,
    });



    println!("{:?}", task_list.tasks);
    println!("---------");
    println!("{:?}", task_list.get_task(3));


}

