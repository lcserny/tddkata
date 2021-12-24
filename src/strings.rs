use std::str::FromStr;

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_empty_string_returns_zero() {
        assert_eq!(0, add(""));
    }

    #[test]
    fn adding_single_number_string_returns_number() {
        assert_eq!(3, add("3"));
        assert_eq!(66, add("66"));
    }

    #[test]
    fn adding_multiple_comma_separated_numbers_returns_sum() {
        assert_eq!(7, add("3,4"));
    }

    #[test]
    fn adding_multiple_newline_separated_numbers_returns_sum() {
        assert_eq!(10, add("9\n1"));
        assert_eq!(15, add("9\n1,5"));
    }

    #[test]
    fn adding_supports_custom_delimiter() {
        assert_eq!(12, add("//;\n6;6"));
        assert_eq!(12, add("//###\n6###6"));
    }
}

pub fn add(numbers: &str) -> i32 {
    if numbers.is_empty() {
        return 0;
    }

    let mut tmp_numbers = String::new();
    let mut delimiters = vec![",", "\n"];

    if numbers.starts_with("//") {
        let idx = numbers.chars().position(|c| c == '\n').unwrap();
        delimiters.push(&numbers[2..idx]);
        tmp_numbers.push_str(&numbers[idx + 1..]);
    } else {
        tmp_numbers.push_str(numbers);
    }

    Regex::new(delimiters.join("|").as_str())
        .expect("invalid delimiter pattern")
        .split(&tmp_numbers)
        .into_iter()
        .map(|t| match i32::from_str(t) {
            Ok(num) => num,
            Err(_) => 0,
        })
        .sum()
}
