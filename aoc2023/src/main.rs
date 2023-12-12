use day1::{Day1Part1, Day1Part2};
use std::{fs, collections::HashMap};
use log::{warn};

use crate::solver::Solver;

mod day1;
mod solver;

const INPUT_FILE_PATH: &'static str = "inputs/day1.txt";
const TEST_INPUT_FILE_PATH: &'static str = "test/data/test_day2.txt";

fn main() {
  env_logger::init();
  // let day1part1 = Day1Part1 {};
  // let res1 = day1part1.solve(INPUT_FILE_PATH);
  // println!("<<<=== Total calibration value Part 1 = [{res1}] ===>>>");

  // let day1part2 = Day1Part2 {};
  // let res2 = day1part2.solve(TEST_INPUT_FILE_PATH);
  // println!("<<<=== Total caatlibration value Part 2 = [{res2}] ===>>>");

  let res3 = solve("test/data/test_day2p1.txt");
  println!("<<<=== Total caatlibration value Day2 p1 = [{res3}] ===>>>");
}

fn solve(file_path: &str) -> u32 {
  let mut map: HashMap<Color, u32> = HashMap::new();

  if let Ok(contents) = fs::read_to_string(file_path) {
    for line in contents.lines() {
      let splits: Vec<&str> = line.split(":").collect();
      let id = splits[0].chars().last().unwrap().to_digit(10);
      let games: Vec<&str> = splits[1].split(";").collect();
      for game in games {
        let color_values: Vec<&str> = game.strip_prefix(" ").unwrap().split(" ").collect();
        println!("{:?}", color_values);
      }
    }
  }
  0
}

struct CubeHand {

}

enum Color {
  RED, GREEN, BLUE
}