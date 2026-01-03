use pomodorotimer::app::gui_app::GuiApp;
use pomodorotimer::core::pomodoro_timer::PomodoroTimer;

fn main() {
    // Create the timer with default values (will be overridden by settings)
    let timer = PomodoroTimer::new(25 * 60, 5 * 60);

    let app = GuiApp::new(timer);
    app.run();
}
