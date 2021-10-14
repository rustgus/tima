//! Parser module
//!
//! Provides parser functions and regular expressions.
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(?P<time>\d*)(?P<unit>[a-zA-Z]*)$").unwrap();
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
                let time = &item["time"];
                if !time.is_empty() {
                    vec_input.push(time.to_string());
                }
                let unit = &item["unit"];
                if !unit.is_empty() {
                    vec_input.push(unit.to_string());
                }
            }
            None => eprintln!("Input is invalid: {}", &item),
        };
    });
    vec_input
}

pub fn normalise_units(entries: Vec<String>) -> Vec<String> {
    entries
        .into_iter()
        .map(|val| match val.as_str() {
            "sec" | "seconds" | "second" | "s" => "s".to_string(),
            "min" | "minute" | "minutes" | "m" => "m".to_string(),
            "hour" | "hours" | "h" => "h".to_string(),
            "milli" | "millis" | "millisecond" | "milliseconds" | "l" => "l".to_string(),
            _ => "".to_string(),
        })
        .filter(|val| val != "")
        .collect()
}

#[macro_export]
macro_rules! vec_string {
    ($($val:expr),*) => {
        {
            let mut values: Vec<String> = Vec::new();
            $(
                values.push($val.to_string());
            )*
            values
        }
    };
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
        let expected: Vec<String> = vec_string!["1", "s", "10", "sec", "101", "milli", "0", "h"];
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

    #[test]
    fn parse_time_only() {
        assert_eq!(vec!["1"], parse(vec!["1".to_string()]));
    }

    #[test]
    fn parse_unit_only() {
        assert_eq!(vec!["milli"], parse(vec!["milli".to_string()]));
    }

    #[test]
    fn normalise_units_with_empty_entries() {
        let entries: Vec<String> = vec![];
        let expected: Vec<String> = vec![];
        let normalised: Vec<String> = normalise_units(entries);
        assert_eq!(expected, normalised);
    }

    #[test]
    fn normalise_units_with_valid_entries() {
        let entries = vec_string!["1", "s", "10", "sec", "101", "milli", "0", "h", "1", "minute"];
        let expected = vec_string!["s", "s", "l", "h", "m"];
        let normalised: Vec<String> = normalise_units(entries);
        assert_eq!(expected, normalised);
    }
}
