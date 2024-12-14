mod algorithms;
mod error;
mod macros;
mod scheduler;
mod task;

use algorithms::PriorityScheduler;
use scheduler::SchedulerBuilder;
use task::{Frequency, Priority, TaskBuilder};

fn main() {
    // Example Usage
    let mut scheduler = SchedulerBuilder::new()
        .algorithm(Box::new(PriorityScheduler::new()))
        .build();

    let task1 = TaskBuilder::new()
        .name("Task 1")
        .frequency(Frequency::Periodic(std::time::Duration::from_secs(1)))
        .priority(Priority::Level5)
        .action(|| {
            info!("Task1 running with priority level 5 repeating every 1 seconds");
        })
        .build();
    let task2 = TaskBuilder::new()
        .name("Task 2")
        .frequency(Frequency::Periodic(std::time::Duration::from_secs(5)))
        .priority(Priority::Level1)
        .action(|| {
            info!("Task2 running with priority level 1 repeating every 5 seconds");
        })
        .build();
    let task3 = TaskBuilder::new()
        .name("Task 3")
        .frequency(Frequency::Periodic(std::time::Duration::from_secs(1)))
        .priority(Priority::Level5)
        .action(|| {
            info!("Task3 running with priority level 5 repeating every 1 seconds");
        })
        .build();

    scheduler.add_task(task1);
    scheduler.add_task(task2);
    scheduler.add_task(task3);
    scheduler.run();
}
