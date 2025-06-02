use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use chrono::Local;
use notify_rust::Notification;
use crate::pomodoro_timer::Period::Today;
use crate::pomodoro_timer::TimerState::{Breaking, Idle, Working};
use crate::timer_commander::TimerCommander;
use crate::timer_database::{create_timer_run, establish_connection, get_timer_runs, get_users};
use crate::timer_runner::{ExitCondition, TimerRunner};

pub struct PomodoroTimer{
    work_duration: Duration,
    break_duration: Duration,
    current_state: Arc<Mutex<TimerState>>,
    commander: Option<TimerCommander>,
    receiver: Option<Receiver<Duration>>,
    username: Option<String>,
}

#[derive(PartialEq, Copy, Eq, Clone, Debug, Hash)]
pub enum TimerState {
    Idle,
    Working,
    Breaking
}

#[derive(PartialEq)]
pub enum Period {
    Today,
    AllTime,
}

impl PomodoroTimer{
    // Constructor that creates a new PomodoroTimer instance
    pub fn new(work_duration_sec: u64, break_duration_sec: u64) -> PomodoroTimer {
        let pomodoro_timer = PomodoroTimer {
            work_duration: Duration::from_secs(work_duration_sec),
            break_duration: Duration::from_secs(break_duration_sec),
            current_state: Arc::new(Mutex::new(Idle)),
            commander: None,
            receiver: None,
            username: None,
        };

        // Create the new timer instance
        pomodoro_timer
    }

    // A run being an "Idle -> Working -> Break -> Idle" iteration
    pub fn start_run(&mut self){
        // Create new Timer runner
        let (tx, rx) = mpsc::channel();
        let (time_tx, time_rx) = mpsc::channel();

        let mut timer_runner = TimerRunner::new(rx, time_tx);

        // Create the command injector
        let timer_commander = TimerCommander::new(tx);

        self.commander = Some(timer_commander);
        self.receiver = Some(time_rx);

        // Save the times in separate variables
        let working_duration = self.work_duration;
        let break_duration = self.break_duration;
        let username = self.username.clone();
        let current_state = Arc::clone(&self.current_state);

        thread::spawn(move || {
            // Run through the phases
            // Start in working phase
            PomodoroTimer::update_state(&current_state, Working);
            let exit_condition = timer_runner.run_timer(working_duration);

            if exit_condition == ExitCondition::Terminated {
                PomodoroTimer::update_state(&current_state, Idle);
                return;
            }

            Notification::new()
                .summary("PomodoroTimer")
                .body("Good work! Take a break before continuing.")
                .show()
                .ok();

            // Then breaking phase
            PomodoroTimer::update_state(&current_state, Breaking);
            let exit_condition = timer_runner.run_timer(break_duration);

            if exit_condition == ExitCondition::Terminated {
                PomodoroTimer::update_state(&current_state, Idle);
                return;
            }

            Notification::new()
                .summary("PomodoroTimer")
                .body("The break is over! Continue with your good work.")
                .show()
                .ok();

            // Then return to idle
            PomodoroTimer::update_state(&current_state, Idle);

            // Log the completed iteration in the database
            if username.is_some() {
                let connection = &mut establish_connection();
                create_timer_run(connection, &*username.unwrap(), &(working_duration.as_secs() as i32), &(break_duration.as_secs() as i32))
            }


        });

    }

    fn update_state(state: &Arc<Mutex<TimerState>>, new_state: TimerState){
        let mut current_state = state.lock().expect("Failed to lock current state");
        *current_state = new_state;
    }

    pub fn get_state(&self) -> TimerState {
        let current_state = self.current_state.lock().expect("Failed to lock current state");
        *current_state
    }

    pub fn start_timer(&mut self){
        let current_state = self.current_state.lock().expect("Failed to lock current state");
        if *current_state == Idle {
            drop(current_state);
            self.start_run();
        } else {
            drop(current_state);
            self.resume_timer()
        }
    }

    pub fn pause_timer(&mut self){
        if self.get_state() == Idle {
            return;
        }

        match &mut self.commander {
            None => println!("Have to start a sessions to give commands"),
            Some(c) => c.pause_timer()
        }
    }

    pub fn stop_timer(&mut self){
        if self.get_state() == Idle {
            return;
        }

        match &mut self.commander {
            None => println!("Have to start a sessions to give commands"),
            Some(c) => {
                PomodoroTimer::update_state(&self.current_state, Idle);
                c.stop_timer();
            }
        }
    }

    pub fn resume_timer(&mut self) {
        match &mut self.commander {
            None => println!("Have to start a sessions to give commands"),
            Some(c) => c.resume_timer()
        }
    }

    pub fn get_work_duration(&self) -> Duration {
        self.work_duration
    }

    pub fn get_break_duration(&self) -> Duration {
        self.break_duration
    }

    pub fn set_work_duration(&mut self, duration: Duration) {
        self.work_duration = duration
    }

    pub fn set_break_duration(&mut self, duration: Duration) {
        self.break_duration = duration
    }

    pub fn get_remaining_time(&mut self) -> Duration {
        if self.receiver.is_some() && self.commander.is_some() {
            let r = self.receiver.as_ref().unwrap();
            let c = self.commander.as_ref().unwrap();
            let success = c.get_time_remaining();
            if !success {
                // We are now in "Idle" since execution stopped
                self.receiver = None;
                self.commander = None;
                return self.get_work_duration();
            }
            // Get the remaining time
            let dur_res = r.try_recv();

            dur_res.unwrap_or(self.get_work_duration())
        } else {
            self.get_work_duration()
        }
    }

    pub fn set_state_time_period(&mut self, period: Duration, state: TimerState) {
        // Stop timer when user updates duration
        self.stop_timer();

        match state {
            Idle => {}
            Working => {self.work_duration = period;},
            Breaking => {self.break_duration = period},
        }
    }

    pub fn get_total_time(&self, period: Period) -> (i32, i32) {
        let connection = &mut establish_connection();

        let user = self.get_username();

        // Check that user is logged in
        if user.is_none() {
            return (0, 0);
        }

        let mut runs = get_timer_runs(connection, &*user.unwrap());

        let cur_date = Local::now().date_naive();

        // Filter out all dates in case only today
        if period == Today {
            runs = runs.into_iter().filter(|tr|{
                tr.date.eq(&cur_date)
            }).collect();
        };

        // Work out the total amount of time used today
        runs.into_iter().fold((0, 0), |acc, tr|{
            let (working, breaking) = acc;
            (working + tr.working_time_secs, breaking + tr.breaking_time_secs)
        })
    }

    // Dummy sign in
    pub fn sign_in(&mut self, username : &str) -> bool {
        if self.get_state() != Idle {
            return false;
        }
        self.username = Some(username.to_string());
        true
    }

    pub fn get_username(&self) -> Option<String> {
        self.username.clone()
    }

    pub fn get_users(&self) -> Vec<String> {
        let connection = &mut establish_connection();
        get_users(connection)
    }
}


