use manager::TaskManager;
use serialized_task_manager::{storage::Storage, *};
use task::{Priority, Task, TaskStatus};

fn main() {
    let mut task = Task::new(
        0,
        String::from("test"),
        TaskStatus::InProgress,
        Priority::Low,
    );

    println!("Initial task: ");
    task.print();

    task.set_status(TaskStatus::Completed);
    task.set_priority(Priority::High);

    println!("Modified task: ");
    task.print();

    if task.is_completed() {
        println!("The task is complete.");
    }

    let mut task_manager = TaskManager::new();
    task_manager.add(task);
    task_manager.add(Task::new(
        1,
        String::from("test2"),
        TaskStatus::InProgress,
        Priority::Low,
    ));
    task_manager.add(Task::new(
        2,
        String::from("test3"),
        TaskStatus::InProgress,
        Priority::Low,
    ));
    task_manager.add(Task::new(
        3,
        String::from("test4"),
        TaskStatus::Completed,
        Priority::Low,
    ));

    println!("\nCurrent task list:");
    task_manager.list_tasks();

    let existing_task = task_manager.get_task(0).unwrap();
    println!("Task id 0 exists:");
    existing_task.print();

    let invalid_task = task_manager.get_task(99);
    if let Some(_task) = invalid_task {
        println!("I actually exist...");
    } else {
        println!("Task id 99 doesn't exist");
    }

    let completed_task = task_manager.complete_task(2);

    match completed_task {
        Ok(task) => {
            println!("This task was completed: ");
            task.print();
        }
        Err(e) => println!("{}", e),
    }

    println!("\nFinal task list:");
    task_manager.list_tasks();

    println!("Saving the task list.");
    let path = std::path::Path::new("./serialized_task_manager").join("tasks.json");
    task_manager.save(&path);

    let keys: Vec<u32> = task_manager
        .container_mut()
        .get_storage_mut()
        .keys()
        .copied()
        .collect();

    for key in keys {
        task_manager.remove(key);
    }

    println!("Emptied in-memory task list:");
    task_manager.list_tasks();

    println!("Loading task manager from json:");
    let loaded_manager = task_manager.load(&path).unwrap();
    loaded_manager.list_tasks();
}
