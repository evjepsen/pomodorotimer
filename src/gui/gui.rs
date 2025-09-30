use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

slint::include_modules!();

fn get_username() -> String {
    let dialog = LoginDialog::new().unwrap();
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
    
    // Use an Rc<RefCell<String>> so the closure can own a clone and mutate it later.
    let username = Rc::new(RefCell::new(String::new()));
    let username_clone = username.clone();
    dialog.on_login({
            let dialog = dialog.as_weak().unwrap();
            move |name, remember| {
            println!("Login with username: {}{}.", name, if remember { ", which will be remembered" } else { "" });
            *username_clone.borrow_mut() = name.to_string();
            dialog.hide();
        }
    });
    dialog.run().unwrap();
    return username.borrow().clone();
}

fn main() -> Result<(), Box<dyn Error>> {
    let username = get_username();
    let ui = AppWindow::new()?;
    ui.on_request_login(|| {
        println!("Login requested from UI");
    });
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;

    Ok(())
}
