# Rust Scheduler Library

This project is a Rust-based library designed for task scheduling, offering features such as priority scheduling and macro-based utilities for efficient and error-resistant task management.

## Features

- **Task Abstraction**: Define and manage tasks easily using the `task.rs` module.
- **Schedulers**: Multiple scheduling strategies, including:
  - **Basic Scheduler**: Found in `scheduler.rs`, providing foundational scheduling logic.
  - **Priority Scheduler**: Located in `algorithms/priority_scheduler.rs`, enabling priority-based task execution.
- **Error Handling**: Robust error management utilities via the `error.rs` module.
- **Macros**: Reusable macros defined in `macros.rs` to simplify repetitive coding tasks.
- **Modular Design**: Clear modularity with a central `algorithms/mod.rs` file.

## Getting Started

### Prerequisites

Ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/benjaminjamesxyz/rust-task-scheduler.git
   cd rust-task-scheduler
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the tests:

   ```bash
   cargo test
   ```

## Usage

### Define a Task
Tasks can be defined using the `task.rs` module and the `TaskBuilder` pattern:

```rust
use crate::task::{TaskBuilder, Frequency, Priority};

let task = TaskBuilder::new()
    .name("Example Task")
    .frequency(Frequency::OneTime)
    .priority(Priority::Level1)
    .action(|| println!("Task executed"))
    .build();
```

### Use the Scheduler

#### Basic Scheduler

```rust
use crate::scheduler::Scheduler;
use algorithms::PriorityScheduler;

let mut scheduler = SchedulerBuilder::new()
    .algorithm(Box::new(PriorityScheduler::new()))
    .build();
scheduler.add_task(task);
scheduler.run();
```

## Project Structure

```
src
├── algorithms
│   ├── mod.rs                  # Central module for algorithms
│   └── priority_scheduler.rs   # Priority-based scheduler
├── error.rs                    # Error handling utilities
├── macros.rs                   # Macro definitions
├── main.rs                     # Entry point for the application
├── scheduler.rs                # Base scheduler implementation
└── task.rs                     # Task definition and management
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature/bugfix:
   ```bash
   git checkout -b feat/feature-name
   ```
3. Commit your changes following conventional commit standards:
   ```bash
   git commit -m "feat: add new feature"
   ```
   Use prefixes like `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, etc., to indicate the type of change.
4. Push to your branch:
   ```bash
   git push origin feat/feature-name
   ```
5. Open a pull request.

## License

This project is licensed under the [GPL v3 License](LICENSE).
