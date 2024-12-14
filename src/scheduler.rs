use crate::task::Task;
use std::collections::BinaryHeap;

pub struct Scheduler {
    tasks: BinaryHeap<Task>,
    algorithm: Box<dyn SchedulerAlgorithm>,
}

impl Scheduler {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn run(&mut self) {
        self.algorithm.run(&mut self.tasks);
    }
}

pub struct SchedulerBuilder {
    algorithm: Option<Box<dyn SchedulerAlgorithm>>,
}

impl SchedulerBuilder {
    pub fn new() -> Self {
        Self { algorithm: None }
    }

    pub fn algorithm(mut self, algorithm: Box<dyn SchedulerAlgorithm>) -> Self {
        self.algorithm = Some(algorithm);
        self
    }

    pub fn build(self) -> Scheduler {
        Scheduler {
            tasks: BinaryHeap::new(),
            algorithm: self.algorithm.expect("Scheduler algorithm is required"),
        }
    }
}

pub trait SchedulerAlgorithm {
    fn run(&self, tasks: &mut BinaryHeap<Task>);
}
