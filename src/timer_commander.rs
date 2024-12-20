use std::sync::mpsc::Sender;

#[derive(Debug, PartialEq)]
pub enum TimerCommand {
    Pause,
    Start,
    Stop
}


pub struct TimerCommander {
    command_sender: Sender<TimerCommand>,
}

impl TimerCommander {
    pub fn new(tx: Sender<TimerCommand>) -> Self {
        TimerCommander {
            command_sender: tx,
        }
    }

    pub fn pause_timer(&mut self) {
        self.command_sender.send(TimerCommand::Pause).unwrap();
    }

    pub fn stop_timer(&mut self) {
        self.command_sender.send(TimerCommand::Stop).unwrap();
    }

}
