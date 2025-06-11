use crate::core::timer_commander::TimerCommand;
use crate::core::timer_commander::TimerCommand::Stop;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

#[derive(PartialEq)]
pub enum ExitCondition {
    Ok,
    Terminated,
}

pub struct TimerRunner {
    command_receiver: Receiver<TimerCommand>,
    time_sender: Sender<Duration>,
}

impl TimerRunner {
    pub fn new(command_receiver: Receiver<TimerCommand>, time_sender: Sender<Duration>) -> Self {
        TimerRunner {
            time_sender,
            command_receiver,
        }
    }

    pub fn run_timer(&mut self, duration: Duration) -> ExitCondition {
        // Start the timer
        let start_time = Instant::now();
        let mut time_in_pause = Duration::new(0, 0);

        while start_time.elapsed() < duration + time_in_pause {
            thread::sleep(Duration::from_millis(10));

            let remaining = (duration + time_in_pause).saturating_sub(start_time.elapsed());

            // Check for new commands
            let command = self.command_receiver.try_recv();

            if let Ok(command) = command {
                match command {
                    TimerCommand::Start => continue,
                    TimerCommand::Pause => {
                        let start_pause = Instant::now();
                        if self.wait_for_resume(remaining) == Stop {
                            return ExitCondition::Terminated;
                        };
                        time_in_pause += start_pause.elapsed();
                    }
                    Stop => return ExitCondition::Terminated,
                    TimerCommand::GetTimeRemaining => self.time_sender.send(remaining).unwrap(),
                }
            }
        }

        ExitCondition::Ok
    }

    fn wait_for_resume(&mut self, remaining: Duration) -> TimerCommand {
        while let Ok(command) = self.command_receiver.recv() {
            match command {
                TimerCommand::Pause => continue,
                TimerCommand::Start => return TimerCommand::Start,
                TimerCommand::GetTimeRemaining => {
                    self.time_sender.send(remaining).unwrap();
                    continue;
                }
                Stop => return Stop,
            }
        }
        Stop
    }
}
