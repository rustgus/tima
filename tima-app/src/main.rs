//! Tima - A simple timer for humans in Rust
//!
//! Using the timer:
//! ```
//! tima <sec|min> [-m]
//! ```
//! where 5 is the number of seconds
//! or minutes, with the `-m` option.
use std::env;
use tima_engine::engine::*;

macro_rules! verbose {
    ($txt:literal, $tmr:expr) => {
        if !$tmr.quiet_mode {
            println!("{}: {} {}", $txt, $tmr.max_count, if $tmr.minutes { "minutes" } else { "seconds" });
        }
    };
}

fn main() {
    let tmr = Tima::init(env::args().skip(1).collect());
    verbose!("Tima started", &tmr);
    tmr.start();
    verbose!("Tima finished", &tmr);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    /// Tests if the quiet mode with value false is working. It 
    /// should not fail.
    fn verbose_with_quiet_mode() {
        let mut tmr = Tima::new();
        tmr.quiet_mode = true;
        verbose!("", &tmr)
    }

    #[test]
    /// Tests if the quiet mode with value true is working. It 
    /// should not fail.
    fn verbose_without_quiet_mode() {
        let mut tmr = Tima::new();
        tmr.max_count = 10;
        verbose!("Tima started", &tmr)
    }
}
