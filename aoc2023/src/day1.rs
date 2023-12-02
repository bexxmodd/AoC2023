use std::fs;
use log::{info, error, warn};

const INPUT_FILE_PATH: &'static str = "inputs/day1.txt";

pub fn solve() -> u32 {
  let mut sum = 0_u32;
  if let Ok(contents) = fs::read_to_string(INPUT_FILE_PATH) {
    for line in contents.lines() {
        let mut digits = Vec::new();
        for char in line.chars() {
          if char.is_digit(10) {
            digits.push(char);
          }
        }
        sum += convert_to_two_digit(&digits);
    }
  } else {
      error!("Failed to open [{INPUT_FILE_PATH}]");
  }

  info!("Total calculated to: [{sum}]");
  sum
}

fn convert_to_two_digit(digits: &Vec<char>) -> u32 {
  return match digits.len() {
    1 => convert_string_to_digit(
      digits.first().unwrap(), digits.first().unwrap()),
    2.. => convert_string_to_digit(
      digits.first().unwrap(), digits.last().unwrap()),
    _ => 0
  }
}

fn convert_string_to_digit(first: &char, second: &char) -> u32 {
  let value = format!("{}{}", first, second);
  if let Ok(res) = value.parse::<u32>() {
    res
  } else {
    warn!("{value} cannot be converted to u32");
    0
  }
}