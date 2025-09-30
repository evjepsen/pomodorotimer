use std::error::Error;

use pomodorotimer::core::pomodoro_timer::PomodoroTimer;

slint::include_modules!();

pub struct App {
    timer: PomodoroTimer,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut pomodoro_app = App {
        timer: PomodoroTimer::new(25, 5),
    };
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
        move || {
            let ui = ui_handle.unwrap();
            println!("Starting timer for user: {}", ui.get_username());
            ui.set_pausable(true);
            pomodoro_app.timer.start_run();

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
