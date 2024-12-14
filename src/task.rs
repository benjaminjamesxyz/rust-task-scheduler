use std::time::Duration;

#[allow(dead_code)]
pub enum Frequency {
    Periodic(Duration),
    OneTime,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Priority {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

#[allow(dead_code)]
pub struct Task {
    pub name: String,
    pub frequency: Frequency,
    pub priority: Priority,
    pub action: Box<dyn Fn() + Send + Sync>,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority) // Higher priority first
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Task {}

pub struct TaskBuilder {
    name: Option<String>,
    frequency: Option<Frequency>,
    priority: Option<Priority>,
    action: Option<Box<dyn Fn() + Send + Sync>>,
}

impl TaskBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            frequency: None,
            priority: None,
            action: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn frequency(mut self, frequency: Frequency) -> Self {
        self.frequency = Some(frequency);
        self
    }

    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn action<F: Fn() + Send + Sync + 'static>(mut self, action: F) -> Self {
        self.action = Some(Box::new(action));
        self
    }

    pub fn build(self) -> Task {
        Task {
            name: self.name.expect("Task name is required"),
            frequency: self.frequency.expect("Frequency is required"),
            priority: self.priority.expect("Priority is required"),
            action: self.action.expect("Action is required"),
        }
    }
}
