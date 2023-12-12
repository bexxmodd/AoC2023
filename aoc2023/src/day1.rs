use std::fs;
use log::{error, info, warn};
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::solver::Solver;

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


pub struct Day1Part1 {}

impl Day1Part1 {
  fn convert_string_to_digit(digits: &Vec<char>) -> u32 {
    let value = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    if let Ok(res) = value.parse::<u32>() {
      res
    } else {
      warn!("{value} cannot be converted to u32");
      0
    }
  }
}

impl Solver for Day1Part1 {
  fn solve(&self, file_path: &str) -> u32 { let mut sum = 0_u32;
    if let Ok(contents) = fs::read_to_string(file_path) {
      for line in contents.lines() {
          let mut digits = Vec::new();
          for char in line.chars() {
            if char.is_digit(10) {
              digits.push(char);
            }
          }
          sum += Day1Part1::convert_string_to_digit(&digits);
      }
    } else {
        error!("Failed to open [{file_path}]");
    }

    info!("Total calculated to: [{sum}]");
    sum
  }
}

pub struct Day1Part2 {}

impl Day1Part2 {
  fn convert_string_to_digit(&self, first: &str, second: &str) -> u32 {
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
}

impl Solver for Day1Part2 {
  fn solve(&self, file_path: &str) -> u32 {
    let mut sum = 0;

    if let Ok(contents) = fs::read_to_string(file_path) {
      for line in contents.lines() {
        let matcher = regex::Regex::new(
          r"([0-9]|one|two|three|four|five|six|seven|eight|nine)"
        ).unwrap();
        let first = matcher.find(line).map(|m| m.as_str()).unwrap();
        
        sum += self.convert_string_to_digit(first, first)
      }
    } else {
      error!("Failed to open [{file_path}]");
    }

    info!("Total calculated to: [{sum}]");
    sum
  }
}