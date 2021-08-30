//! Parser module
//!
//! Provides parser functions and regular expressions.
use std::cmp::Ordering;

fn parse<'a>(text: &'a String) -> Vec<&'a str> {
    text.split("").filter(|chr| chr.cmp(&"") != Ordering::Equal).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_arguments() {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let values = args.into_iter().reduce(|a, b| a + &" " + &b).unwrap_or_default();
        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, parse(&values));
    }

    #[test]
    fn test_parse_no_time() {
        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, parse(&"".to_string()));
    }

    #[test]
    fn test_parse_time_in_seconds() {
        assert_eq!(vec!["1", "s"], parse(&"1s".to_string()));
    }
}
