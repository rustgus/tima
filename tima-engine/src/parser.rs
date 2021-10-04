//! Parser module
//!
//! Provides parser functions and regular expressions.
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(?P<time>\d+)(?P<unit>[a-zA-Z]*)$").unwrap();
}

pub struct Components {
    pub values: Vec<u32>,
    pub units: Vec<String>,
}

pub fn parse(text: Vec<String>) -> Vec<String> {
    let mut vec_input: Vec<String> = vec![];
    text.into_iter().for_each(|item| {
        let input = RE.captures(&item);
        match input {
            Some(item) => {
                vec_input.push(item["time"].to_string());
                vec_input.push(item["unit"].to_string());
            }
            None => eprintln!("{:?}", input),
        };
    });
    vec_input
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_1s() {
        let text: Vec<String> = [
            "1s".to_string(),
            "10sec".to_string(),
            "101milli".to_string(),
            "0h".to_string(),
            "1s1".to_string(),
        ]
        .to_vec();
        let expected: Vec<String> = ["1", "s", "10", "sec", "101", "milli", "0", "h"]
            .iter()
            .map(|value| value.to_string())
            .collect();
        assert_eq!(expected, parse(text));
    }

    #[test]
    fn no_arguments() {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, parse(args));
    }

    #[test]
    fn parse_no_time() {
        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, parse(vec!["".to_string()]));
    }

    #[test]
    fn parse_time_in_seconds() {
        assert_eq!(vec!["1", "s"], parse(vec!["1s".to_string()]));
    }
}
