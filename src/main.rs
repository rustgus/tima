fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
pub struct Timer {
    max_count: u64,
}

impl Timer {
    pub fn new(max_count: u64) -> Self {
        Timer {
            max_count: max_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::args;
    use super::*;

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
