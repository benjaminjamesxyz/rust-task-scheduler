# Rust Task Scheduler

A priority-based task scheduler implemented in Rust, featuring support for periodic and one-time task execution using a binary heap. This project provides a simple yet flexible way to manage and execute tasks based on customizable priorities and intervals.

## Features

- **Priority-based execution**: Tasks with higher priorities are executed first.
- **Periodic tasks**: Set intervals for tasks to run repeatedly.
- **One-time tasks**: Execute tasks only once.
- **Dynamic task addition**: Add tasks with different priorities and configurations during runtime.
- **Efficient scheduling**: Uses a `BinaryHeap` for optimized priority handling.

## Usage

### Prerequisites

- Install [Rust](https://www.rust-lang.org/) on your system.

### Cloning the Repository

```bash
git clone https://github.com/your-username/rust-task-scheduler.git
cd rust-task-scheduler
```

### Running the Scheduler

1. Open the `main` function in `src/main.rs` to customize the tasks.
2. Run the project:

```bash
cargo run
```

### Example Output

The following is an example of the scheduler in action:

```plaintext
Task 3: Execute only once
Task 2: 2 sec delay with prio level 1
Task 4: 2 sec delay with prio level 1
Task 1: Send ARP request
```

## Code Overview

### Core Components

- **`EventLoop`**:
  - Manages the task queue and executes tasks based on priority and interval.

- **`PriorityTask`**:
  - Represents a task with metadata such as priority, interval, and execution status.

- **`TaskBuilder`**:
  - Provides a convenient API for creating tasks with various configurations.

### Adding Tasks

Tasks are added to the event loop using the `TaskBuilder` structure. Here's an example:

```rust
let task = TaskBuilder::new(|| println!("Hello, World!"))
    .with_interval(Duration::from_secs(10))
    .with_priority(2);
event_loop.add_task(task);
```

## Contributing

Contributions are welcome! If you have ideas for improvements or encounter issues, please:

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Submit a pull request with a detailed description.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions or feedback, feel free to reach out:

- **Email**: benworksxyz-github@yahoo.com
- **GitHub**: [benjaminjamesxyz](https://github.com/benjaminjamesxyz)
