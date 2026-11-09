use crate::storage::{Storage, StorageBackend};
use crate::task::{Task, TaskStatus};

pub struct TaskManager {
    memory: StorageBackend<u32, Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            memory: StorageBackend::new(),
        }
    }

    pub fn add(&mut self, task: Task) {
        self.memory.insert(task.get_id(), task);
    }

    pub fn remove(&mut self, id: u32) -> Option<Task> {
        let result = self.memory.remove(id);
        match result {
            Some(task) => Some(task),
            None => None,
        }
    }

    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.memory.retrieve(id)
    }

    pub fn get_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.memory.retrieve_mut(id)
    }

    pub fn complete_task(&mut self, id: u32) -> Result<&Task, &str> {
        let result = self.get_task_mut(id);
        if let Some(task) = result {
            if task.is_completed() {
                Err("The task is already completed.")
            } else {
                task.set_status(TaskStatus::Completed);
                Ok(task)
            }
        } else {
            Err("The task ID doesn't exist.")
        }
    }

    pub fn list_tasks(&self) {
        for row in self.memory.get_storage() {
            row.1.print();
        }
    }
}
