mod pomodoro_timer_tests {
    use pomodorotimer::core::pomodoro_timer::PomodoroTimer;
    use pomodorotimer::core::pomodoro_timer::TimerState::{Breaking, Idle, Working};
    use std::thread;
    use std::time::Duration;

    // Timer runner tests
    #[test]
    fn should_new_timer_start_in_idle() {
        // Given a timer
        let timer = PomodoroTimer::new(0, 0);

        // When I ask for it's state
        let got = timer.get_state();

        // Then it should be in Idle state
        assert_eq!(got, Idle)
    }

    #[test]
    fn should_timer_end_in_idle_state() {
        // Given a timer
        let mut timer = PomodoroTimer::new(0, 0);

        // When I ask it to start and wait second
        timer.start_run();

        // Then it should be in Idle state
        let got = timer.get_state();
        assert_eq!(got, Idle)
    }

    #[test]
    fn should_pause_in_working() {
        // Given a timer
        let mut timer = PomodoroTimer::new(2, 1);
        timer.start_run();
        thread::sleep(Duration::from_secs(1));

        // When I ask it to pause in working
        timer.pause_timer();

        // Then it should remain in the same state
        assert_eq!(timer.get_state(), Working);
    }

    #[test]
    fn should_pause_in_breaking() {
        // Given a timer
        let mut timer = PomodoroTimer::new(0, 2);
        timer.start_run();
        thread::sleep(Duration::from_secs(1));

        // When I ask it to pause in breaking
        timer.pause_timer();

        // Then it should remain in the same state
        assert_eq!(timer.get_state(), Breaking);
    }

    #[test]
    fn should_pause_and_resume_stop_in_idle() {
        // Given a timer
        let mut timer = PomodoroTimer::new(2, 1);
        timer.start_run();
        thread::sleep(Duration::from_secs(1));

        // When I ask it to pause and resume
        timer.pause_timer();
        timer.resume_timer();

        // Then it should remain in the same state
        assert_eq!(timer.get_state(), Working);
    }

    #[test]
    fn should_stop_end_in_idle() {
        // Given a timer
        let mut timer = PomodoroTimer::new(10, 5);
        timer.start_run();

        // When I ask it to stop
        timer.stop_timer();
        let state = timer.get_state();

        // Then it should return to "Idle"
        assert_eq!(state, Idle);
    }

    #[test]
    fn should_pause_stop_timer_runner() {
        // Given a timer
        let mut timer = PomodoroTimer::new(10, 2);
        timer.start_run();

        // When I ask it to pause
        let time_rem_before = timer.get_remaining_time();
        timer.pause_timer();
        thread::sleep(Duration::from_secs(2));

        // The when I start it again, it should have the same remaining time
        timer.start_timer();
        let time_rem_after = timer.get_remaining_time();
        assert!(time_rem_before - time_rem_after < Duration::from_secs(1));
    }
}
