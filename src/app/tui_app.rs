use crate::app::tui_app::MessageType::{InvalidCommand, ValidCommand};
use crate::core::pomodoro_timer::Period::{AllTime, Today};
use crate::core::pomodoro_timer::{PomodoroTimer, TimerState};
use crossterm::event::poll;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text,
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};
use std::time::Duration;
use tui_input::{Input, InputRequest};

pub struct App {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<(String, MessageType)>,
    /// The pomodoro timer - Application
    timer: PomodoroTimer,
    prev_message: usize,
}

enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq)]
enum MessageType {
    ValidCommand,
    Information,
    InvalidCommand,
}

impl App {
    pub fn new(timer: PomodoroTimer) -> Self {
        App {
            input: "".into(),
            input_mode: InputMode::Normal,
            messages: vec![],
            timer,
            prev_message: 0,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) {
        loop {
            terminal
                .draw(|frame| self.draw(frame))
                .expect("Could not draw");

            if let Ok(true) = poll(Duration::from_millis(100)) {
                if let Ok(Event::Key(key)) = event::read() {
                    match self.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('e') => {
                                self.input_mode = InputMode::Editing;
                            }
                            KeyCode::Char('q') => {
                                return;
                            }
                            _ => {}
                        },
                        InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                            KeyCode::Enter => self.submit_command(),
                            KeyCode::Char(to_insert) => self.enter_char(to_insert),
                            KeyCode::Backspace => self.delete_char(),
                            KeyCode::Left => self.move_cursor_left(),
                            KeyCode::Right => self.move_cursor_right(),
                            KeyCode::Up => self.choose_prev_command(KeyCode::Up),
                            KeyCode::Down => self.choose_prev_command(KeyCode::Down),

                            KeyCode::Esc => self.input_mode = InputMode::Normal,
                            _ => {}
                        },
                        InputMode::Editing => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [information_area, help_area, input_area, messages_area] = vertical.areas(frame.area());

        // Timer display in top half
        let time_remaining = self.timer.get_remaining_time().as_secs();
        let time_remaining_min = time_remaining / 60;
        let time_remaining_sec = time_remaining % 60;

        let work_duration = self.timer.get_work_duration().as_secs();
        let work_duration_min = work_duration / 60;
        let work_duration_sec = work_duration % 60;

        let break_duration = self.timer.get_break_duration().as_secs();
        let break_duration_min = break_duration / 60;
        let break_duration_sec = break_duration % 60;

        let timer_text = vec![
            text::Line::from(format!(
                "Time remaining: {:02}:{:02}",
                time_remaining_min, time_remaining_sec
            )),
            text::Line::from(""),
            text::Line::from(format!("Timer state: {:?}", self.timer.get_state())),
            text::Line::from(""),
            text::Line::from(format!(
                "Current stats: Working {:02}:{:02} and breaking {:02}:{:02}",
                work_duration_min, work_duration_sec, break_duration_min, break_duration_sec
            )),
        ];

        let timer_widget = Paragraph::new(timer_text)
            .block(Block::bordered().title("Timer"))
            .style(Style::default().fg(Color::Green));

        frame.render_widget(timer_widget, information_area);

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start entering commands.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };

        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.to_string())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        match self.input_mode {
            InputMode::Normal => {}
            InputMode::Editing => frame.set_cursor_position(Position::new(
                input_area.x + self.input.cursor() as u16 + 1,
                input_area.y + 1,
            )),
        }

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .rev()
            .enumerate()
            .map(|(_, m)| {
                let (message, t) = m;
                let content = Line::from(Span::raw(format!("{message}")));
                let color = match t {
                    ValidCommand => Color::Green,
                    InvalidCommand => Color::Red,
                    MessageType::Information => Color::Yellow,
                };
                ListItem::new(content).style(Style::default().fg(color))
            })
            .collect();
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_widget(messages, messages_area);
    }

    fn submit_command(&mut self) {
        let message = self.input.to_string();
        let mut reply: Option<String> = None;
        let message_array: Vec<&str> = message.split_whitespace().collect();

        let command_validity = match message_array.first() {
            Some(&"start") => {
                self.timer.start_timer();
                ValidCommand
            }
            Some(&"stop") => {
                self.timer.stop_timer();
                ValidCommand
            }
            Some(&"pause") => {
                self.timer.pause_timer();
                ValidCommand
            }
            Some(&"help") => {
                reply = Some(String::from("Commands: Start, Stop, Pause, Set <state> <duration in min>, stats <today, all-time>, login <user-name>, whoami, users"));
                ValidCommand
            }
            Some(&"set") => {
                let mut command_validity = ValidCommand;
                let state_to_update = match message_array.get(1) {
                    Some(&"working") => Some(TimerState::Working),
                    Some(&"breaking") => Some(TimerState::Breaking),
                    _ => {
                        reply = Some(String::from(
                            "Can only set the time for working or breaking. ",
                        ));
                        command_validity = InvalidCommand;
                        None
                    }
                };

                let time_in_min = message_array.get(2);
                let mut new_time_amount = None;

                match time_in_min {
                    Some(time) => {
                        let new_time = time.parse::<f32>();
                        if let Ok(time) = new_time {
                            new_time_amount = Some(time);
                        } else {
                            reply = Some(String::from("Invalid time"));
                        }
                    }
                    None => command_validity = InvalidCommand,
                }

                if new_time_amount.is_some() && state_to_update.is_some() {
                    let time_in_min = new_time_amount.unwrap() * 60.0;
                    let period = Duration::from_secs(time_in_min.floor() as u64);
                    self.timer
                        .set_state_time_period(period, state_to_update.unwrap())
                }

                command_validity
            }
            Some(&"stats") => {
                let when = message_array.get(1);

                let (work_dur, break_dur) = match when {
                    Some(&"today") => self.timer.get_total_time(Today),
                    Some(&"all-time") => self.timer.get_total_time(AllTime),
                    _ => (-1, -1),
                };

                reply = Some(format!(
                    "{:?}: Total work duration: {:?} minutes. Total break duration: {:?} minutes",
                    self.timer.get_username().unwrap_or("None".to_string()),
                    work_dur / 60,
                    break_dur / 60
                ));
                ValidCommand
            }
            Some(&"login") => {
                let username = message_array.get(1);

                match username {
                    Some(username) => {
                        let success = self.timer.sign_in(username);
                        if success {
                            reply = Some(String::from("You are signed in!"))
                        } else {
                            reply = Some(String::from("Something went wrong trying to sign you in"))
                        }
                    }
                    None => {
                        reply = Some(String::from("Enter a username please"));
                    }
                };

                ValidCommand
            }
            Some(&"whoami") => {
                let username = self.timer.get_username();
                reply = match username {
                    Some(u) => Some(format!("You are signed in as {:?}", u)),
                    None => Some(String::from("You are not signed in")),
                };
                ValidCommand
            }
            Some(&"users") => {
                let users = self.timer.get_users();
                reply = Some(format!("Users: {:?}", users));
                ValidCommand
            }
            _ => InvalidCommand,
        };

        self.messages.push((message, command_validity));

        if reply.is_some() {
            self.messages
                .push((reply.unwrap(), MessageType::Information))
        }

        // Clear the terminal
        self.input = "".into();
        self.prev_message = 0;
    }

    fn enter_char(&mut self, char_entered: char) {
        let input_request = InputRequest::InsertChar(char_entered);
        let input_response = self.input.handle(input_request);

        if input_response.is_some() {
            self.move_cursor_right();
        }
    }

    fn delete_char(&mut self) {
        let delete_request = InputRequest::DeletePrevChar;
        self.input.handle(delete_request);
    }

    fn move_cursor_left(&mut self) {
        let move_left_request = InputRequest::GoToPrevChar;
        self.input.handle(move_left_request);
    }

    fn move_cursor_right(&mut self) {
        let move_left_request = InputRequest::GoToNextChar;
        self.input.handle(move_left_request);
    }

    fn choose_prev_command(&mut self, code: KeyCode) {
        let prev_message: Vec<String> = self
            .messages
            .iter()
            .filter(|(_, val)| *val == ValidCommand)
            .map(|(m, _)| m.clone())
            .rev()
            .collect();

        match code {
            KeyCode::Up => self.prev_message = self.prev_message.saturating_add(1),
            KeyCode::Down => self.prev_message = self.prev_message.saturating_sub(1),
            _ => {}
        }

        let idx = self.prev_message;

        if idx == 0 {
            self.input = "".into();
        } else {
            match prev_message.get(idx.saturating_sub(1)) {
                Some(msg) => {
                    self.input = msg.clone().into();
                }
                None => {}
            }
        }
    }
}
