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
}

/// Method implementation for the Timer struct.
impl Timer {
    /// `new` method implementation, with `max_count` mandatory.
    pub fn new(max_count: u64) -> Self {
        Timer {
            max_count: max_count,
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
    }

    #[test]
    fn test_config() {
        for arg in args() {
            println!("\n{}", arg);
        }
    }
}
