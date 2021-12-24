use std::str::FromStr;

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_empty_string_returns_zero() {
        assert_eq!(0, add("").unwrap());
    }

    #[test]
    fn adding_single_number_string_returns_number() {
        assert_eq!(3, add("3").unwrap());
        assert_eq!(66, add("66").unwrap());
    }

    #[test]
    fn adding_multiple_comma_separated_numbers_returns_sum() {
        assert_eq!(7, add("3,4").unwrap());
    }

    #[test]
    fn adding_multiple_newline_separated_numbers_returns_sum() {
        assert_eq!(10, add("9\n1").unwrap());
        assert_eq!(15, add("9\n1,5").unwrap());
    }

    #[test]
    fn adding_supports_custom_delimiter() {
        assert_eq!(12, add("//;\n6;6").unwrap());
        assert_eq!(12, add("//#\n6#6").unwrap());
    }

    #[test]
    fn adding_negative_numbers_not_allowed() {
        assert_eq!(Err(String::from("negatives not allowed: -1")), add("6,-1"));
    }

    #[test]
    fn adding_big_numbers_ignores_them() {
        assert_eq!(12, add("//;\n12;1003").unwrap());
    }

    #[test]
    fn adding_custom_delimiter_can_be_multiple_chars() {
        assert_eq!(12, add("//[###]\n6###6").unwrap());
    }
}

pub fn add(numbers: &str) -> Result<i32, String> {
    if numbers.is_empty() {
        return Ok(0);
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

    let iter: Vec<i32> = Regex::new(delimiters.join("|").as_str())
        .expect("invalid delimiter pattern")
        .split(&tmp_numbers)
        .map(|t| match i32::from_str(t) {
            Ok(num) => num,
            Err(_) => 0,
        })
        .collect();

    let mut sum = 0;
    for x in iter {
        if x.is_negative() {
            return Err(format!("negatives not allowed: {}", x));
        }
        if x <= 1000 {
            sum += x
        }
    }
    Ok(sum)
}
