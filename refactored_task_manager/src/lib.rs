pub mod manager;
pub mod storage;
pub mod task;

#[cfg(test)]
mod tests {
    use crate::manager::*;
    use crate::storage::*;
    use crate::task::*;

    #[test]
    fn task_testing() {
        let mut task = Task::new(
            0,
            "testing tasks".to_string(),
            TaskStatus::Pending,
            Priority::Low,
        );

        task.set_priority(Priority::High);
        task.set_status(TaskStatus::Completed);

        assert_eq!(*task.get_priority(), Priority::High);
        assert_eq!(*task.get_status(), TaskStatus::Completed);
        assert!(task.is_completed());
    }

    #[test]
    fn task_manager_initializes_empty() {
        let manager = TaskManager::new();

        assert_eq!(manager.container().get_storage().len(), 0);
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
        let new_task = manager.get_task(1).unwrap();
        assert_eq!(manager.container().get_storage().len(), 1);
        assert_eq!(new_task.get_id(), 1);
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

        let removed = manager.remove(1);

        assert!(removed.is_some());
        assert_eq!(removed.unwrap().get_id(), 1);
        assert_eq!(manager.container().get_storage().len(), 0);
    }

    #[test]
    fn task_manager_remove_from_empty_returns_none() {
        let mut manager = TaskManager::new();

        assert!(manager.remove(1).is_none());
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
        assert_eq!(task.unwrap().get_id(), 2);
    }

    #[test]
    fn task_manager_returns_none_for_unknown_id() {
        let manager = TaskManager::new();

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
        assert_eq!(*result.unwrap().get_status(), TaskStatus::Completed);
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
