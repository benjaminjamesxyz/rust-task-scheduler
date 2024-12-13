use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

// A type alias for tasks that are functions that can be called mutably and sent across threads
// These tasks are stored as `Box` to allow dynamic dispatch
// `FnMut` allows the closure to modify captured variables
// `Send` ensures the closure can be safely transferred between threads
type Task = Box<dyn FnMut() + Send>;

// Represents a task with a priority and scheduling metadata
struct PriorityTask {
    priority: usize, // Determines the execution order; higher priorities are executed first
    task: Task,      // The actual task to execute
    last_run: Instant, // The last time this task was run
    interval: Option<Duration>, // Optional interval for periodic tasks
    one_time: bool,  // Indicates if the task should only execute once
    executed: bool,  // Tracks whether the task has already executed (for one-time tasks)
}

// Ordering based on task priority; higher priority comes first
impl Ord for PriorityTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority) // Reverse order for max-heap behavior
    }
}

impl PartialOrd for PriorityTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PriorityTask {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PriorityTask {}

// The main event loop structure
struct EventLoop {
    task_queue: BinaryHeap<PriorityTask>, // A max-heap for managing tasks by priority
}

impl EventLoop {
    // Creates a new event loop with an empty task queue
    fn new() -> Self {
        Self {
            task_queue: BinaryHeap::new(),
        }
    }

    // Adds a new task to the event loop
    fn add_task(&mut self, task: TaskBuilder) {
        // Extract task details from the builder
        let interval = task.interval;
        let one_time = task.one_time;
        let priority = task.priority;
        let task_fn = task.build();

        // Push the task into the queue with its metadata
        self.task_queue.push(PriorityTask {
            priority,
            task: task_fn,
            last_run: Instant::now(),
            interval,
            one_time,
            executed: false,
        });
    }

    // Runs the event loop until all tasks are completed
    fn run(&mut self) {
        while !self.task_queue.is_empty() {
            let mut temp_queue = BinaryHeap::new();

            // Process each task in the queue
            while let Some(mut priority_task) = self.task_queue.pop() {
                if priority_task.one_time && priority_task.executed {
                    // Skip tasks that have already executed if they are one-time
                    continue;
                }

                if let Some(interval) = priority_task.interval {
                    // Check if the task's interval has elapsed
                    if priority_task.last_run.elapsed() >= interval {
                        (priority_task.task)(); // Execute the task
                        priority_task.last_run = Instant::now();
                        if priority_task.one_time {
                            priority_task.executed = true; // Mark as executed
                        }
                    } else {
                        // Requeue the task if its interval hasn't elapsed
                        temp_queue.push(priority_task);
                        continue;
                    }
                } else {
                    // Execute tasks with no interval immediately
                    (priority_task.task)();
                    if priority_task.one_time {
                        priority_task.executed = true;
                    }
                }

                // Push the task back into the queue if it's not done
                temp_queue.push(priority_task);
            }

            // Update the main task queue
            self.task_queue = temp_queue;
        }
    }
}

// A builder for creating tasks with optional metadata
struct TaskBuilder {
    task_fn: Option<Box<dyn FnMut() + Send>>, // The function to execute
    interval: Option<Duration>,               // Optional interval for periodic tasks
    one_time: bool,                           // Indicates if the task is one-time
    priority: usize,                          // Task priority
}

impl TaskBuilder {
    // Creates a new task builder with a specified function
    fn new<F>(task_fn: F) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        Self {
            task_fn: Some(Box::new(task_fn)),
            interval: None,
            one_time: false,
            priority: 0,
        }
    }

    // Sets the interval for the task
    fn with_interval(mut self, duration: Duration) -> Self {
        self.interval = Some(duration);
        self
    }

    // Marks the task as one-time
    fn one_time(mut self) -> Self {
        self.one_time = true;
        self
    }

    // Sets the priority for the task
    fn with_priority(mut self, priority: usize) -> Self {
        self.priority = priority;
        self
    }

    // Builds the task, consuming the builder
    fn build(self) -> Task {
        let mut task_fn = self.task_fn.unwrap();
        Box::new(move || {
            task_fn();
        })
    }
}

fn main() {
    // Initialize the event loop
    let mut event_loop = EventLoop::new();

    // Define Task 1 with an interval and priority
    let task1 = TaskBuilder::new(|| println!("Task 1: Send ARP request"))
        .with_interval(Duration::from_secs(60 * 60))
        .with_priority(0);

    // Define Task 2 with an interval and priority
    let task2 = TaskBuilder::new(|| println!("Task 2: 2 sec delay with prio level 1"))
        .with_interval(Duration::from_secs(2))
        .with_priority(1);

    // Define Task 3 as a one-time task with a priority
    let task3 = TaskBuilder::new(|| println!("Task 3: Execute only once"))
        .one_time()
        .with_priority(3);

    // Define Task 4 with an interval and priority
    let task4 = TaskBuilder::new(|| println!("Task 4: 2 sec delay with prio level 1"))
        .with_interval(Duration::from_secs(2))
        .with_priority(1);

    // Add tasks to the event loop
    event_loop.add_task(task1);
    event_loop.add_task(task2);
    event_loop.add_task(task3);
    event_loop.add_task(task4);

    // Start the event loop
    event_loop.run();
}
