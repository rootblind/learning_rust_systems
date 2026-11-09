use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

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

    pub fn to_string(&self) -> serde_json::Result<String> {
        let result = serde_json::to_string(self.memory.get_storage())?;
        Ok(result)
    }

    pub fn save(&self, path: &PathBuf) {
        let json = self.to_string();
        if let Ok(json) = json {
            let mut data_file = File::create(path).expect("Failed to create the json.");
            data_file
                .write(json.as_bytes())
                .expect("Failed to write in json.");
        } else {
            panic!("Failed to serialize the manager.");
        }
    }

    pub fn load(self, path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut json = File::open(path)?;
        let mut json_content = String::new();
        json.read_to_string(&mut json_content)
            .expect("Failed to read the json content.");

        let memory_map: HashMap<u32, Task> = serde_json::from_str(&json_content)?;

        Ok(TaskManager {
            memory: StorageBackend::from(memory_map),
        })
    }

    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.memory.retrieve(id)
    }

    pub fn get_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.memory.retrieve_mut(id)
    }

    pub fn container(&self) -> &StorageBackend<u32, Task> {
        &self.memory
    }

    pub fn container_mut(&mut self) -> &mut StorageBackend<u32, Task> {
        &mut self.memory
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
