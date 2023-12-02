mod day1;

fn main() {
  env_logger::init();
  let res = day1::solve();
  println!("<<<=== Total calibration value = [{res}] ===>>>");
}