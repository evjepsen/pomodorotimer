use std::{error::Error, rc::Rc};

use pomodorotimer::core::pomodoro_timer::{PomodoroTimer, TimerState};

slint::include_modules!();

pub struct App {
    timer: PomodoroTimer,
}

fn main() -> Result<(), Box<dyn Error>> {
    let pomodoro_app = Rc::new(std::cell::RefCell::new(App {
        timer: PomodoroTimer::new(25*60, 5*60),
    }));
    let mut paused = false;

    let ui = AppWindow::new()?;
    let dialog = LoginDialog::new()?;

    ui.on_logout({
        let dialog = dialog.as_weak().unwrap();
        move || {
            println!("Logout.");
            dialog.show().unwrap();
        }
    });

    ui.on_start({
        let ui_handle = ui.as_weak();
        let pomodoro_app = pomodoro_app.clone();
        move || {
            let ui = ui_handle.unwrap();
            println!("Starting timer for user: {}", ui.get_username());
            ui.set_pausable(true);
            pomodoro_app.borrow_mut().timer.start_run();
        }
    });

    ui.on_pause_resume({
        let pomodoro_app = pomodoro_app.clone();

        move || {
            println!("Pause/Resume clicked");
            paused = !paused;
            let timer = &mut pomodoro_app.borrow_mut().timer;
            if !paused {
                timer.resume_timer();
            } else {
                timer.pause_timer();
            }        
        }
    });

    ui.on_tick({
        let ui = ui.as_weak().unwrap();
        let ref_cell = pomodoro_app.clone();

        move || {
            let mut app = ref_cell.borrow_mut();
            let remaining = app.timer.get_remaining_time();
            let minutes = remaining.as_secs() / 60;
            let seconds = remaining.as_secs() % 60;
            let time_string = format!("{:02}:{:02}", minutes, seconds);
            let status_string = match app.timer.get_state() {
                TimerState::Idle => "Idle",
                TimerState::Working => "Running",
                TimerState::Breaking => "On Break",
            };
            ui.set_remaining(time_string.into());
            ui.set_status(status_string.to_string().into());
            if matches!(
                app.timer.get_state(),
                TimerState::Working | TimerState::Breaking
            ) {
                ui.set_pausable(true);
            } else {
                ui.set_pausable(false);
            }
        }
    });

    ui.show()?;

    dialog.on_check_ok({
        let dialog_handle = dialog.as_weak();
        let dialog = dialog_handle.unwrap();
        move |text| {
            if text.len() > 0 {
                dialog.set_ok_button_enabled(true);
            } else {
                dialog.set_ok_button_enabled(false);
            }
        }
    });

    dialog.on_login({
        let dialog = dialog.as_weak().unwrap();
        move |name, remember| {
            println!(
                "Login with username: {}{}.",
                name,
                if remember {
                    ", which will be remembered"
                } else {
                    ""
                }
            );

            dialog.hide().expect("Failed to hide dialog");

            ui.set_username(name);
        }
    });

    dialog.show()?;

    slint::run_event_loop()?;
    Ok(())
}
