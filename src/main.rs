//! TimerIt - A simple timer in Rust
//!
//! Using the timer:
//! ```
//! timer <sec|min> [-m]
//! ```
//! where 5 is the number of seconds
//! or minutes, with the `-m` option.
mod timer;

use std::env;

fn main() {
    let tmr = crate::timer::Timer::init(env::args().collect());
    println!("Timer started");
    tmr.start();
    println!("Timer finished");
}
