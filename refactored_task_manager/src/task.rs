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

    pub fn get_id(&self) -> u32 {
        self.id
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
