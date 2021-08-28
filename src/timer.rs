//! Timer module
//!
//! Provides the Timer struct and methods
use std::thread::sleep;
use std::time::Duration;

/// Timer struct, with the `max_count` variable
/// to be set with the number or seconds or minutes.
///
/// Example:
/// ```
/// # use timer::timer::Timer;
/// let tmr = Timer::new(12);
/// assert_eq!(12, tmr.max_count);
/// assert!(!tmr.minutes);
/// ```
#[derive(Debug)]
pub struct Timer {
    pub max_count: u64,
    pub minutes: bool,
}

/// Method implementation for the Timer struct.
impl Timer {
    /// `new` method implementation, with `max_count` mandatory.
    pub fn new(max_count: u64) -> Self {
        Timer {
            max_count: max_count,
            minutes: false,
        }
    }

    /// Initialises a new Timer with the command
    /// line arguments passed in the argument `args`.
    pub fn init(args: Vec<String>) -> crate::timer::Timer {
        let mut tmr = crate::timer::Timer::new(0);
        for (index, arg) in args.iter().enumerate() {
            if index > 0 {
                if arg == "-m" {
                    eprintln!("Time will be set in minutes!");
                    tmr.minutes = true;
                } else if tmr.max_count == 0 {
                    match arg.parse::<u64>() {
                        Ok(n) => tmr.max_count = n,
                        Err(e) => eprintln!("Error while parsing argument {}: {}", arg, e),
                    };
                }
            }
        }
        tmr
    }

    /// Starts the timer with the given time
    pub fn start(&self) {
        let time = if self.minutes {
            self.max_count * 60
        } else {
            self.max_count
        };
        sleep(Duration::from_secs(time));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::args;

    #[test]
    fn test_timer_new() {
        let tmr = Timer::new(12);
        assert_eq!(12, tmr.max_count);
        assert!(!tmr.minutes);
    }

    #[test]
    fn test_new_with_minutes() {
        let mut tmr = Timer::new(12);
        tmr.minutes = true;
        assert_eq!(12, tmr.max_count);
        assert!(tmr.minutes);
    }

    #[test]
    fn test_config() {
        for arg in args() {
            println!("\n{}", arg);
        }
    }

    #[test]
    fn test_start() {
        let tmr = Timer::new(1);
        tmr.start();
    }

    #[test]
    fn test_init_no_values() {
        let args: Vec<String> = vec![];
        let tmr = Timer::init(args);
        assert_eq!(0, tmr.max_count);
    }

    #[test]
    fn test_init() {
        let args: Vec<String> = vec![
            String::from(""),
            "-m".to_string(),
            "12".to_string(),
            "13".to_string(),
        ];
        let tmr = Timer::init(args);
        assert_eq!(12, tmr.max_count);
        assert!(tmr.minutes);
    }
}
