use pomodorotimer::app::tui_app::App;
use pomodorotimer::core::pomodoro_timer::PomodoroTimer;

/// Personal pomodoro timer main function
fn main() {
    // Create the timer
    let timer = PomodoroTimer::new(20 * 60, 5 * 60);

    // Run the TUI
    let terminal = ratatui::init();
    App::new(timer).run(terminal);
    ratatui::restore();
}
