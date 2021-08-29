//! Tima module
//!
//! Provides the Tima struct and methods
use std::thread::sleep;
use std::time::Duration;

/// Tima struct, with the `max_count` variable
/// to be set with the number or seconds or minutes.
///
/// Example:
/// ```
/// # use tima::tima::Tima;
/// let tmr = Tima::new(12);
/// assert_eq!(12, tmr.max_count);
/// assert!(!tmr.minutes);
/// ```
#[derive(Debug)]
pub struct Tima {
    pub max_count: u64,
    pub minutes: bool,
}

/// Method implementation for the Timer struct.
impl Tima {
    /// `new` method implementation, with `max_count` mandatory.
    pub fn new(max_count: u64) -> Self {
        Tima {
            max_count,
            minutes: false,
        }
    }

    /// Initialises a new Timer with the command
    /// line arguments passed in the argument `args`.
    pub fn init(args: Vec<String>) -> Self {
        let mut tmr = crate::tima::Tima::new(0);
        args.into_iter()
            .filter(|arg| arg == "-m" || !arg.parse::<u64>().is_err())
            .for_each(|arg| {
                if arg == "-m" {
                    tmr.minutes = true;
                } else if tmr.max_count == 0 {
                    tmr.max_count = arg.parse::<u64>().unwrap();
                }
            });
        tmr
    }

    /// Starts the timer with the underlying time
    /// in Tima::max_count.
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

    #[test]
    fn test_tima() {
        let tmr = Tima {
            max_count: 12,
            minutes: false,
        };
        assert_eq!(12, tmr.max_count);
    }

    #[test]
    fn test_tima_new() {
        let tmr = Tima::new(12);
        assert_eq!(12, tmr.max_count);
        assert!(!tmr.minutes);
    }

    #[test]
    fn test_new_with_minutes() {
        let mut tmr = Tima::new(12);
        tmr.minutes = true;
        assert_eq!(12, tmr.max_count);
        assert!(tmr.minutes);
    }

    #[test]
    fn test_start() {
        let tmr = Tima::new(1);
        tmr.start();
    }

    #[test]
    fn test_init_no_values() {
        let args: Vec<String> = vec![];
        let tmr = Tima::init(args);
        assert_eq!(0, tmr.max_count);
    }

    #[test]
    fn test_init() {
        let args: Vec<String> = vec![
            String::from(""),
            "-m".to_string(),
            "12".to_string(),
            "c".to_string(),
        ];
        let tmr = Tima::init(args);
        assert_eq!(12, tmr.max_count);
        assert!(tmr.minutes);
    }
}
