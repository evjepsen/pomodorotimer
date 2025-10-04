use std::{error::Error, rc::Rc};

use pomodorotimer::core::pomodoro_timer::{PomodoroTimer, TimerState};
use pomodorotimer::config::Config;

slint::include_modules!();

pub struct App {
    timer: PomodoroTimer,
}

fn main() -> Result<(), Box<dyn Error>> {
    let pomodoro_app = Rc::new(std::cell::RefCell::new(App {
        timer: PomodoroTimer::new(25*60, 5*60),
    }));
    let mut paused = false;

    // Load configuration
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Warning: Could not load config: {}. Using defaults.", e);
        Config::default()
    });

    let ui = AppWindow::new()?;
    let dialog = LoginDialog::new()?;

    // Check if we have a saved username
    let show_dialog = if let Some(ref saved_username) = config.default_user {
        if !saved_username.is_empty() {
            // Set the username directly and skip the dialog
            ui.set_username(saved_username.clone().into());
            false // Don't show dialog
        } else {
            true // Show dialog
        }
    } else {
        true // Show dialog
    };

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
        let ui = ui.as_weak().unwrap();
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

            // Save username to config if remember is checked
            if remember {
                let mut config = Config::load().unwrap_or_default();
                if let Err(e) = config.set_default_user(Some(name.to_string())) {
                    eprintln!("Warning: Could not save username to config: {}", e);
                }
            } else {
                // If remember is not checked, clear any saved username
                let mut config = Config::load().unwrap_or_default();
                if let Err(e) = config.set_default_user(None) {
                    eprintln!("Warning: Could not clear username from config: {}", e);
                }
            }

            dialog.hide().expect("Failed to hide dialog");
            ui.set_username(name);
        }
    });

    // Only show the login dialog if we don't have a saved username
    if show_dialog {
        dialog.show()?;
    }

    slint::run_event_loop()?;
    Ok(())
}
