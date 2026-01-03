# Pomodoro Timer AI Instructions

## Architecture Overview
This is a Rust-based Pomodoro timer with a TUI interface and SQLite persistence.

- **Core Logic (`src/core/`)**:
  - `PomodoroTimer`: Main entry point for timer logic. Manages state and spawns runners.
  - `TimerRunner`: Runs in a dedicated thread to handle the countdown. Communicates via `mpsc` channels.
  - `TimerCommander`: Sends commands (`Start`, `Stop`, `Pause`, `GetTimeRemaining`) to the `TimerRunner`.
- **UI (`src/app/`)**:
  - `TuiApp`: Ratatui-based terminal interface. Handles user input and renders the timer state.
- **Database (`src/db/`)**:
  - `TimerDatabase`: Diesel-based SQLite integration. Handles migrations automatically on connection.

## Critical Workflows
- **Run App**: `cargo run` (requires `DATABASE_URL` in `.env` or environment).
- **Run Tests**: `cargo test` (integration tests in `tests/pomodoro_timer_tests.rs`).
- **Database**: Migrations are in `migrations/` and are embedded in the binary via `diesel_migrations`.

## Project Patterns & Conventions
- **Threading**: The timer countdown runs in a separate thread spawned by `PomodoroTimer::start_run`.
- **Communication**: Use `std::sync::mpsc` for sending commands to the runner and receiving time updates.
- **State Management**: `TimerState` (Idle, Working, Breaking) is shared using `Arc<Mutex<TimerState>>` to allow the UI and the background thread to access/update it.
- **Error Handling**: Uses `unwrap()` or `expect()` in many places for simplicity, but `anyhow` is available in dependencies.
- **Notifications**: Uses `notify-rust` for desktop notifications when a phase ends.
- **Database**: Uses Diesel with SQLite. Migrations are embedded and run on startup in `establish_connection`.

## Key Files
- [src/core/pomodoro_timer.rs](src/core/pomodoro_timer.rs): Orchestrates the timer phases and manages `TimerState`.
- [src/core/timer_runner.rs](src/core/timer_runner.rs): The actual countdown loop running in a separate thread.
- [src/app/tui_app.rs](src/app/tui_app.rs): TUI implementation using `ratatui` and `tui-input`.
- [src/db/timer_database.rs](src/db/timer_database.rs): Database connection and CRUD operations.

## Example: Adding a Command
1. Add the command to `TimerCommand` enum in [src/core/timer_commander.rs](src/core/timer_commander.rs).
2. Update `TimerRunner::run_timer` in [src/core/timer_runner.rs](src/core/timer_runner.rs) to handle the new command.
3. Add a helper method to `TimerCommander` to send the command.
4. Update `App::submit_command` in [src/app/tui_app.rs](src/app/tui_app.rs) to parse and trigger the command.
