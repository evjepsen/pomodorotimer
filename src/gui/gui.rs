use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let dialog = LoginDialog::new()?;

    ui.on_logout({
        let dialog = dialog.as_weak().unwrap();
        move || {
            println!("Logout.");
            dialog.show().unwrap();
        }
    });
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
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
