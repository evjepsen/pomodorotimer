use crate::core::pomodoro_timer::{PomodoroTimer, TimerState, Period};
use crate::core::settings::AppSettings;
use fltk::{
    app,
    button::Button,
    frame::Frame,
    group::{Flex, Pack, PackType},
    input::{Input, SecretInput},
    prelude::*,
    window::Window,
    enums::{Color, Font, FrameType},
};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use fltk::dialog::alert;

pub struct GuiApp {
    timer: Rc<RefCell<PomodoroTimer>>,
    settings: Rc<RefCell<AppSettings>>,
}

impl GuiApp {
    pub fn new(timer: PomodoroTimer) -> Self {
        let settings = AppSettings::load();
        let mut timer = timer;
        
        // Apply settings to timer
        timer.set_work_duration(settings.get_work_duration());
        timer.set_break_duration(settings.get_break_duration());
        if let Some(ref username) = settings.username {
            timer.sign_in(username);
        }

        Self {
            timer: Rc::new(RefCell::new(timer)),
            settings: Rc::new(RefCell::new(settings)),
        }
    }

    pub fn run(&self) {
        let app = app::App::default();
        let mut wind = Window::default()
            .with_size(400, 500)
            .with_label("Pomodoro Timer");

        let mut pack = Pack::default().with_size(380, 480).center_of(&wind);
        pack.set_spacing(10);

        // --- Login Section ---
        let mut login_pack = Pack::default().with_size(380, 120);
        login_pack.set_spacing(5);
        Frame::default().with_size(0, 25).with_label("Login");
        let mut user_input = Input::default().with_size(0, 30).with_label("User: ");
        if let Some(ref u) = self.settings.borrow().username {
            user_input.set_value(u);
        }
        let mut pass_input = SecretInput::default().with_size(0, 30).with_label("Pass: ");
        if let Some(ref p) = self.settings.borrow().password {
            pass_input.set_value(p);
        }
        let mut login_btn = Button::default().with_size(0, 30).with_label("Login / Save Credentials");
        login_pack.end();

        // --- Timer Section ---
        let mut timer_pack = Pack::default().with_size(380, 150);
        timer_pack.set_spacing(10);
        let mut timer_display = Frame::default().with_size(0, 60).with_label("00:00");
        timer_display.set_label_size(40);
        timer_display.set_label_font(Font::CourierBold);

        let mut btn_flex = Flex::default().with_size(380, 40).row();
        let mut start_btn = Button::default().with_label("Start");
        let mut pause_btn = Button::default().with_label("Pause");
        let mut stop_btn = Button::default().with_label("Stop");
        btn_flex.end();
        
        let mut status_frame = Frame::default().with_size(0, 20).with_label("Idle");
        timer_pack.end();

        // --- Settings Section ---
        let mut settings_pack = Pack::default().with_size(380, 100);
        settings_pack.set_spacing(5);
        Frame::default().with_size(0, 25).with_label("Settings (minutes)");
        let mut work_input = Input::default().with_size(0, 30).with_label("Work: ");
        work_input.set_value(&(self.settings.borrow().work_duration_secs / 60).to_string());
        let mut break_input = Input::default().with_size(0, 30).with_label("Break: ");
        break_input.set_value(&(self.settings.borrow().break_duration_secs / 60).to_string());
        let mut save_settings_btn = Button::default().with_size(0, 30).with_label("Save Settings");
        settings_pack.end();

        // --- Stats Section ---
        let mut stats_frame = Frame::default().with_size(380, 40).with_label("Stats: Login to see");
        
        pack.end();
        wind.end();
        wind.show();

        // Callbacks
        let timer_c = self.timer.clone();
        let settings_c = self.settings.clone();
        login_btn.set_callback(move |b| {
            let user = user_input.value();
            let pass = pass_input.value();
            let mut s = settings_c.borrow_mut();
            s.username = Some(user.clone());
            s.password = Some(pass);
            s.save();
            timer_c.borrow_mut().sign_in(&user);
            b.set_label("Credentials Saved");
        });

        let timer_c = self.timer.clone();
        start_btn.set_callback(move |_| {
            let mut t = timer_c.borrow_mut();
            if !t.is_user_signed_in() {
                alert(100, 100, "Please login first");
                return;
            }
            t.start_timer();
        });

        let timer_c = self.timer.clone();
        pause_btn.set_callback(move |_| {
            timer_c.borrow_mut().pause_timer();
        });

        let timer_c = self.timer.clone();
        stop_btn.set_callback(move |_| {
            timer_c.borrow_mut().stop_timer();
        });

        let timer_c = self.timer.clone();
        let settings_c = self.settings.clone();
        save_settings_btn.set_callback(move |_| {
            let work_min: u64 = work_input.value().parse().unwrap_or(25);
            let break_min: u64 = break_input.value().parse().unwrap_or(5);
            
            let mut s = settings_c.borrow_mut();
            s.work_duration_secs = work_min * 60;
            s.break_duration_secs = break_min * 60;
            s.save();

            let mut t = timer_c.borrow_mut();
            t.set_work_duration(Duration::from_secs(s.work_duration_secs));
            t.set_break_duration(Duration::from_secs(s.break_duration_secs));
        });

        // Update loop
        let timer_c = self.timer.clone();
        let mut last_stats_update = std::time::Instant::now();
        app::add_timeout3(0.1, move |handle| {
            let mut t = timer_c.borrow_mut();
            let state = t.get_state();
            let remaining = t.get_remaining_time();
            
            let mins = remaining.as_secs() / 60;
            let secs = remaining.as_secs() % 60;
            timer_display.set_label(&format!("{:02}:{:02}", mins, secs));
            
            status_frame.set_label(&format!("{:?}", state));
            match state {
                TimerState::Working => status_frame.set_label_color(Color::Red),
                TimerState::Breaking => status_frame.set_label_color(Color::Green),
                TimerState::Idle => status_frame.set_label_color(Color::Foreground),
            }

            if t.is_user_signed_in() && (last_stats_update.elapsed() > Duration::from_secs(5) || state == TimerState::Idle) {
                let (w, b) = t.get_total_time(Period::Today);
                stats_frame.set_label(&format!("Today: Work {}m, Break {}m", w/60, b/60));
                last_stats_update = std::time::Instant::now();
            }

            app::repeat_timeout3(0.1, handle);
        });

        app.run().unwrap();
    }
}
