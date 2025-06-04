use std::sync::mpsc::Sender;

#[derive(Debug, PartialEq)]
pub enum TimerCommand {
    Pause,
    Start,
    Stop,
    GetTimeRemaining,
}

pub struct TimerCommander {
    command_sender: Sender<TimerCommand>,
}

impl TimerCommander {
    pub fn new(tx: Sender<TimerCommand>) -> Self {
        TimerCommander { command_sender: tx }
    }

    pub fn pause_timer(&mut self) {
        self.command_sender.send(TimerCommand::Pause).unwrap();
    }

    pub fn stop_timer(&mut self) {
        self.command_sender.send(TimerCommand::Stop).unwrap();
    }

    pub fn resume_timer(&mut self) {
        self.command_sender.send(TimerCommand::Start).unwrap();
    }

    pub fn get_time_remaining(&self) -> bool {
        match self.command_sender.send(TimerCommand::GetTimeRemaining) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
