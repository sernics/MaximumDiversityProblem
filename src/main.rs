mod points;
mod parse;
mod mdp_problem;
mod mdp_solution;
mod mdp_greedy;
mod mdp;

use crate::mdp_problem::MDPProblem;
use crate::mdp::MDP;

fn main() {
  let path = std::path::PathBuf::from(
    std::env::args().nth(1).expect("No file path provided"));
  let points: MDPProblem = parse::parse_file(&path);
  println!("{}", points);
  const SIZE_M: u8 = 3;
  let mut greedy = mdp_greedy::MDPGreedy::new(points, SIZE_M);
  println!("{}", greedy.execute());
}