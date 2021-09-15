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

fn main() {
    let tmr = Tima::init(env::args().skip(1).collect());
    println!("Tima started");
    tmr.start();
    println!("Tima finished");
}
