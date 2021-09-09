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
/// # use tima::engine::Tima;
/// let tmr = Tima::new(12);
/// assert_eq!(12, tmr.max_count);
/// assert!(!tmr.minutes);
/// ```
#[derive(Debug, Default)]
pub struct Tima {
    pub max_count: u64,
    pub minutes: bool,
}

struct Value {
    a_str: String,
    a_num: u64,
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
        args.into_iter()
            .map(Tima::convert_arguments)
            .filter(Tima::valid_values)
            .map(Tima::set_values)
            .reduce(Tima::create_final_tima)
            .unwrap_or_default()
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

    fn valid_values(value: &Value) -> bool {
        value.a_str == "-m" || value.a_str == "-q" || value.a_num > 0
    }

    fn set_values(value: Value) -> Tima {
        let mut tmr = Tima::new(0);
        if value.a_str == "-m" {
            tmr.minutes = true;
        }
        tmr.max_count = value.a_num;
        tmr
    }

    fn create_final_tima(tmr1: Tima, mut tmr2: Tima) -> Tima {
        if tmr1.max_count > tmr2.max_count {
            tmr2.max_count = tmr1.max_count
        }
        if tmr1.minutes {
            tmr2.minutes = true
        }
        tmr2
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
        let args: Vec<String> = vec!["-m".to_string(), "12".to_string(), "c".to_string()];
        let tmr = Tima::init(args);
        assert_eq!(12, tmr.max_count);
        assert!(tmr.minutes);
    }

    #[test]
    fn test_init_repeated_values() {
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
}
