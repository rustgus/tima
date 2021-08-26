//! TimerIt - A simple timer in Rust
//!
//! Using the timer:
//! ```
//! timer <sec|min> [-m]
//! ```
//! where 5 is the number of seconds
//! or minutes, with the `-m` option.

fn main() {
    println!("Hello, world!");
}

/// Timer struct, with the `max_count` variable
/// to be set with the number or seconds or minutes.
#[derive(Debug)]
pub struct Timer {
    max_count: u64,
    minutes: bool,
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

    /// `new_with_minutes` method implementation, with `max_count` mandatory
    /// and the time set in minutes
    pub fn new_with_minutes(max_count: u64) -> Self {
        Timer {
            max_count: max_count,
            minutes: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::args;

    #[test]
    fn test_timer_new() {
        let timer = Timer::new(12);
        assert_eq!(12, timer.max_count);
        assert!(!timer.minutes);
    }

    #[test]
    fn test_config() {
        for arg in args() {
            println!("\n{}", arg);
        }
    }
}
