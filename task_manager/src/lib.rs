#[derive(PartialEq, Debug)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(PartialEq, Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Pending => write!(f, "Pending"),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "High"),
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
        }
    }
}

#[derive(Debug)]
pub struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
    priority: Priority,
}

impl Task {
    pub fn new(id: u32, description: String, status: TaskStatus, priority: Priority) -> Task {
        Task {
            id,
            description,
            status,
            priority,
        }
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    pub fn print(&self) {
        println!(
            "ID: {} | Description: {} | Status: {} | Priority: {}",
            self.id, self.description, self.status, self.priority
        )
    }

    pub fn is_completed(&self) -> bool {
        match self.status {
            TaskStatus::Completed => true,
            _ => false,
        }
    }
}

pub struct TaskManager {
    list: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { list: vec![] }
    }

    pub fn add(&mut self, task: Task) {
        self.list.push(task);
    }

    pub fn remove(&mut self) -> Option<Task> {
        let result = self.list.pop();
        match result {
            Some(task) => Some(task),
            None => None,
        }
    }

    pub fn get_task(&mut self, id: u32) -> Option<&mut Task> {
        let result = self.list.iter_mut().find(|task| task.id == id);
        match result {
            Some(task) => Some(task),
            None => None,
        }
    }

    pub fn complete_task(&mut self, id: u32) -> Result<&Task, &str> {
        let result = self.get_task(id);
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
        for task in &self.list {
            task.print();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_is_working() {
        let mut task = Task::new(
            0,
            String::from("test"),
            TaskStatus::InProgress,
            Priority::High,
        );
        task.set_status(TaskStatus::Completed);
        task.set_priority(Priority::Low);
        assert_eq!(task.status, TaskStatus::Completed);
        assert_eq!(task.priority, Priority::Low);
        assert_eq!(task.is_completed(), true);
    }

    #[test]
    fn task_manager_initializes_empty() {
        let manager = TaskManager::new();

        assert_eq!(manager.list.len(), 0);
    }

    #[test]
    fn task_manager_adds_task() {
        let mut manager = TaskManager::new();

        let task = Task::new(
            1,
            String::from("Test task"),
            TaskStatus::InProgress,
            Priority::High,
        );

        manager.add(task);

        assert_eq!(manager.list.len(), 1);
        assert_eq!(manager.list[0].id, 1);
    }

    #[test]
    fn task_manager_removes_task() {
        let mut manager = TaskManager::new();

        let task = Task::new(
            1,
            String::from("Test task"),
            TaskStatus::InProgress,
            Priority::High,
        );

        manager.add(task);

        let removed = manager.remove();

        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id, 1);
        assert_eq!(manager.list.len(), 0);
    }

    #[test]
    fn task_manager_remove_from_empty_returns_none() {
        let mut manager = TaskManager::new();

        assert!(manager.remove().is_none());
    }

    #[test]
    fn task_manager_gets_task_by_id() {
        let mut manager = TaskManager::new();

        manager.add(Task::new(
            1,
            String::from("First task"),
            TaskStatus::InProgress,
            Priority::High,
        ));

        manager.add(Task::new(
            2,
            String::from("Second task"),
            TaskStatus::InProgress,
            Priority::Low,
        ));

        let task = manager.get_task(2);

        assert!(task.is_some());
        assert_eq!(task.unwrap().id, 2);
    }

    #[test]
    fn task_manager_returns_none_for_unknown_id() {
        let mut manager = TaskManager::new();

        assert!(manager.get_task(99).is_none());
    }

    #[test]
    fn task_manager_completes_task() {
        let mut manager = TaskManager::new();

        manager.add(Task::new(
            1,
            String::from("Complete me"),
            TaskStatus::InProgress,
            Priority::High,
        ));

        let result = manager.complete_task(1);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, TaskStatus::Completed);
    }

    #[test]
    fn task_manager_cannot_complete_task_twice() {
        let mut manager = TaskManager::new();

        manager.add(Task::new(
            1,
            String::from("Already completed"),
            TaskStatus::Completed,
            Priority::High,
        ));

        let result = manager.complete_task(1);

        assert_eq!(result.unwrap_err(), "The task is already completed.");
    }

    #[test]
    fn task_manager_complete_nonexistent_task() {
        let mut manager = TaskManager::new();

        let result = manager.complete_task(99);

        assert_eq!(result.unwrap_err(), "The task ID doesn't exist.");
    }
}
