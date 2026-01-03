use crate::core::pomodoro_timer::{Period, PomodoroTimer, TimerState};
use crate::core::settings::AppSettings;
use fltk::dialog::alert;
use fltk::{
    app,
    button::Button,
    enums::{Color, Font},
    frame::Frame,
    group::{Flex, Pack},
    input::Input,
    prelude::*,
    window::Window,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

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
            .with_size(350, 450)
            .with_label("Pomodoro Timer");

        let mut pack = Pack::default().with_size(330, 430).center_of(&wind);
        pack.set_spacing(10);

        let (user_input, login_btn) = self.create_login_section();
        let (timer_display, start_btn, pause_btn, stop_btn, status_frame) =
            self.create_timer_section();
        let (work_input, break_input, save_settings_btn) = self.create_settings_section();
        let stats_frame = Frame::default()
            .with_size(330, 30)
            .with_label("Stats: Login to see");

        pack.end();
        wind.end();
        wind.show();

        self.setup_callbacks(
            user_input,
            login_btn,
            start_btn,
            pause_btn,
            stop_btn,
            work_input,
            break_input,
            save_settings_btn,
        );

        self.setup_update_loop(timer_display, status_frame, stats_frame);

        app.run().unwrap();
    }

    fn create_login_section(&self) -> (Input, Button) {
        let mut login_pack = Pack::default().with_size(330, 80);
        login_pack.set_spacing(5);
        Frame::default().with_size(0, 20).with_label("Login");

        let user_row = Flex::default().with_size(330, 25).row();
        Frame::default().with_size(60, 25).with_label("User:");
        let mut user_input = Input::default();
        user_row.end();

        if let Some(ref u) = self.settings.borrow().username {
            user_input.set_value(u);
        }
        let login_btn = Button::default()
            .with_size(0, 25)
            .with_label("Login / Save Username");
        login_pack.end();
        (user_input, login_btn)
    }

    fn create_timer_section(&self) -> (Frame, Button, Button, Button, Frame) {
        let mut timer_pack = Pack::default().with_size(330, 130);
        timer_pack.set_spacing(10);
        let mut timer_display = Frame::default().with_size(0, 50).with_label("00:00");
        timer_display.set_label_size(36);
        timer_display.set_label_font(Font::CourierBold);

        let btn_flex = Flex::default().with_size(330, 35).row();
        let start_btn = Button::default().with_label("Start");
        let pause_btn = Button::default().with_label("Pause");
        let stop_btn = Button::default().with_label("Stop");
        btn_flex.end();

        let status_frame = Frame::default().with_size(0, 20).with_label("Idle");
        timer_pack.end();
        (timer_display, start_btn, pause_btn, stop_btn, status_frame)
    }

    fn create_settings_section(&self) -> (Input, Input, Button) {
        let mut settings_pack = Pack::default().with_size(330, 110);
        settings_pack.set_spacing(5);
        Frame::default()
            .with_size(0, 20)
            .with_label("Settings (minutes)");

        let work_row = Flex::default().with_size(330, 25).row();
        Frame::default().with_size(60, 25).with_label("Work:");
        let mut work_input = Input::default();
        work_row.end();

        let break_row = Flex::default().with_size(330, 25).row();
        Frame::default().with_size(60, 25).with_label("Break:");
        let mut break_input = Input::default();
        break_row.end();

        work_input.set_value(&(self.settings.borrow().work_duration_secs / 60).to_string());
        break_input.set_value(&(self.settings.borrow().break_duration_secs / 60).to_string());

        let save_settings_btn = Button::default()
            .with_size(0, 25)
            .with_label("Save Settings");
        settings_pack.end();
        (work_input, break_input, save_settings_btn)
    }

    fn setup_callbacks(
        &self,
        user_input: Input,
        mut login_btn: Button,
        mut start_btn: Button,
        mut pause_btn: Button,
        mut stop_btn: Button,
        work_input: Input,
        break_input: Input,
        mut save_settings_btn: Button,
    ) {
        let timer_c = self.timer.clone();
        let settings_c = self.settings.clone();
        login_btn.set_callback(move |b| {
            let user = user_input.value();
            let mut s = settings_c.borrow_mut();
            s.username = Some(user.clone());
            s.save();
            timer_c.borrow_mut().sign_in(&user);
            b.set_label("Username Saved");
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
    }

    fn setup_update_loop(
        &self,
        mut timer_display: Frame,
        mut status_frame: Frame,
        mut stats_frame: Frame,
    ) {
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

            if t.is_user_signed_in()
                && (last_stats_update.elapsed() > Duration::from_secs(5)
                    || state == TimerState::Idle)
            {
                let (w, b) = t.get_total_time(Period::Today);
                stats_frame.set_label(&format!("Today: Work {}m, Break {}m", w / 60, b / 60));
                last_stats_update = std::time::Instant::now();
            }

            app::repeat_timeout3(0.1, handle);
        });
    }
}
