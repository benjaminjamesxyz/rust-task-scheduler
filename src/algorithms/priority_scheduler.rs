use crate::scheduler::SchedulerAlgorithm;
use crate::task::{Frequency, Task};
use std::collections::BinaryHeap;
use std::thread;
use std::time::{Duration, Instant};

pub struct PriorityScheduler;

impl PriorityScheduler {
    pub fn new() -> Self {
        Self
    }
}

impl SchedulerAlgorithm for PriorityScheduler {
    fn run(&self, tasks: &mut BinaryHeap<Task>) {
        let mut periodic_tasks = Vec::new();

        loop {
            // Check and execute tasks in the queue
            while let Some(task) = tasks.pop() {
                let start = Instant::now();
                (task.action)();

                // If the task is periodic, calculate the next execution time
                if let Frequency::Periodic(duration) = task.frequency {
                    periodic_tasks.push((task, start + duration));
                }
            }

            // Check for periodic tasks ready to be re-added
            let now = Instant::now();
            let mut to_re_add = Vec::new();

            for (task, next_run) in periodic_tasks.drain(..) {
                if now >= next_run {
                    tasks.push(task);
                } else {
                    to_re_add.push((task, next_run));
                }
            }

            // Add remaining periodic tasks back
            periodic_tasks.extend(to_re_add);

            // Sleep briefly to avoid busy waiting
            if tasks.is_empty() && periodic_tasks.is_empty() {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
