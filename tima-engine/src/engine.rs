//! Tima engine module
//!
//! Provides the Tima struct and methods
use std::thread::sleep;
use std::time::Duration;

/// Tima struct, with the `max_count` variable
/// to be set with the number or seconds or minutes.
///
/// Example:
/// ```
/// # use tima_engine::engine::Tima;
/// let mut tmr = Tima::new();
/// tmr.max_count = 12;
/// assert_eq!(12, tmr.max_count);
/// # assert!(!tmr.minutes);
/// ```
#[derive(Debug, Default)]
pub struct Tima {
    pub max_count: u64,
    pub minutes: bool,
    pub quiet_mode: bool,
}

struct Value {
    a_str: String,
    a_num: u64,
}

/// Method implementation for the Timer struct.
impl Tima {
    /// `new` method implementation, with `max_count` mandatory.
    pub fn new() -> Self {
        Tima {
            max_count: 0,
            minutes: false,
            quiet_mode: false,
        }
    }

    /// Initialises a new Timer with the command
    /// line arguments passed in the argument `args`.
    pub fn init(args: Vec<String>) -> Self {
        let mut tmr = Tima::new();
        args.into_iter()
            .map(Tima::convert_arguments)
            .for_each(Tima::create_tima(&mut tmr));
        tmr
    }

    fn convert_arguments(arg: String) -> Value {
        match arg.parse::<u64>() {
            Ok(n) => Value {
                a_str: "".to_string(),
                a_num: n,
            },
            Err(_) => Value {
                a_str: arg,
                a_num: 0,
            },
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_lifetimes))]
    fn create_tima<'r>(tmr: &'r mut Self) -> impl FnMut(Value) + 'r {
        move |value: Value| {
            if value.a_str == "-m" {
                tmr.minutes = true;
            } else if value.a_str == "-q" {
                tmr.quiet_mode = true;
            } else if value.a_num > 0 {
                tmr.max_count = value.a_num;
            }
        }
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
    fn tima() {
        let tmr = Tima {
            max_count: 12,
            minutes: false,
            quiet_mode: false,
        };
        assert_eq!(12, tmr.max_count);
    }

    #[test]
    fn tima_new() {
        let tmr = Tima::new();
        assert_eq!(0, tmr.max_count);
        assert!(!tmr.minutes);
        assert!(!tmr.quiet_mode);
    }

    #[test]
    fn new_with_minutes() {
        let mut tmr = Tima::new();
        tmr.minutes = true;
        assert_eq!(0, tmr.max_count);
        assert!(tmr.minutes);
    }

    #[test]
    fn start() {
        let mut tmr = Tima::new();
        tmr.max_count = 1;
        tmr.start();
    }

    #[test]
    fn init_no_values() {
        let args: Vec<String> = vec![];
        let tmr = Tima::init(args);
        assert_eq!(0, tmr.max_count);
    }

    #[test]
    fn init() {
        let args: Vec<String> = vec!["-m".to_string(), "12".to_string(), "c".to_string()];
        let tmr = Tima::init(args);
        assert_eq!(12, tmr.max_count);
        assert!(tmr.minutes);
    }

    #[test]
    fn init_repeated_values() {
        let args: Vec<String> = vec![
            "-m".to_string(),
            "12".to_string(),
            "24".to_string(),
            "-q".to_string(),
        ];
        let tmr = Tima::init(args);
        assert_eq!(24, tmr.max_count);
        assert!(tmr.minutes);
    }

    #[test]
    fn init_quiet_mode() {
        let args: Vec<String> = vec![
            "-m".to_string(),
            "12".to_string(),
            "24".to_string(),
            "-q".to_string(),
        ];
        let tmr = Tima::init(args);
        assert!(tmr.quiet_mode);
    }
}
