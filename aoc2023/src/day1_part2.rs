use std::fs;
use log::{error, info, warn};
use lazy_static::lazy_static;
use std::collections::HashMap;

const INPUT_FILE_PATH: &'static str = "inputs/day1.txt";
const TEST_INPUT_FILE_PATH: &'static str = "test/data/test_day2.txt";

lazy_static! {
  static ref WORD_TO_DIGIT_DICT: HashMap<&'static str, char> = {
    let mut map = HashMap::new();
    map.insert("one", '1');
    map.insert("two", '2');
    map.insert("three", '3');
    map.insert("four", '4');
    map.insert("five", '5');
    map.insert("six", '6');
    map.insert("seven", '7');
    map.insert("eight", '8');
    map.insert("nine", '9');
    map.insert("1", '1');
    map.insert("2", '2');
    map.insert("3", '3');
    map.insert("4", '4');
    map.insert("5", '5');
    map.insert("6", '6');
    map.insert("7", '7');
    map.insert("8", '8');
    map.insert("9", '9');
    map
  };
}

pub fn solve() -> u32 {
  let mut sum = 0;

  if let Ok(contents) = fs::read_to_string(INPUT_FILE_PATH) {
    for line in contents.lines() {
      let matcher = regex::Regex::new(
        r#"[0-9]|one|two|three|four|five|six|seven|eight|nine"#
      ).unwrap();
      let matches: Vec<&str> = matcher.find_iter(line).map(|m| m.as_str()).collect();
      let first = *matches.first().unwrap();
      let second = *matches.last().unwrap();
      sum += convert_string_to_digit(first, second);
    }
  } else {
    error!("Failed to open [{INPUT_FILE_PATH}]");
  }

  info!("Total calculated to: [{sum}]");
  sum
}

fn convert_string_to_digit(first: &str, second: &str) -> u32 {
  let _first = WORD_TO_DIGIT_DICT.get(first).unwrap();
  let _second = WORD_TO_DIGIT_DICT.get(second).unwrap();
  let value = format!("{}{}", _first, _second);
  if let Ok(res) = value.parse::<u32>() {
    res
  } else {
    warn!("{value} cannot be converted to u32");
    0
  }
}