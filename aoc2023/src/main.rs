mod day1_part1;
mod day1_part2;

fn main() {
  env_logger::init();
  let res1 = day1_part1::solve();
  println!("<<<=== Total calibration value Part 1 = [{res1}] ===>>>");

  let res2 = day1_part2::solve();
  println!("<<<=== Total calibration value Part 2 = [{res2}] ===>>>");
}