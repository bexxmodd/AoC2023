/// A trait for solving Advent Of Code problems based on a file path.
pub trait Solver {
  /// Solves the Advent of Code problem based on the contents of the file at the
  ///  given `file_path`.
  fn solve(&self, file_path: &str) -> u32;
}