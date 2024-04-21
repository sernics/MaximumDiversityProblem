mod points;
mod parse;
mod mdp_problem;
mod mdp_solution;
mod mdp_greedy;
mod mdp_grasp;
mod mdp;
mod environment;

use crate::mdp_problem::MDPProblem;
use crate::mdp::MDP;

fn main() {
  let path = std::path::PathBuf::from(
    std::env::args().nth(1).expect("No file path provided"));
  let points: MDPProblem = parse::parse_file(&path);
  println!("\x1b[31mPoints:\x1b[0m");
  println!("{}", points);
  const SIZE_M: u8 = 3;
  let mut greedy = mdp_greedy::MDPGreedy::new(points.clone(), SIZE_M); // Clone the points variable
  let result = greedy.execute();
  println!("\x1b[31mGreedy:\x1b[0m");
  println!("{}", result);
  println!("\x1b[31mDiversity greedy:\x1b[0m {}", result.calculate_diversity());
  let mut env = environment::Environment::new(points.clone(), result.clone());
  let new_solution = env.swap();
  println!("\x1b[31mSolution after swap:\x1b[0m");
  println!("{}", new_solution);
  println!("\x1b[31mDiversity:\x1b[0m {}", new_solution.calculate_diversity());
  let mut grasp = mdp_grasp::MDPGrasp::new(points.clone(), SIZE_M);
  let result = grasp.execute();
  println!("\x1b[31mGrasp:\x1b[0m");
  println!("{}", result);
  println!("\x1b[31mDiversity grasp:\x1b[0m {}", result.calculate_diversity());
  let mut env = environment::Environment::new(points, result.clone());
  let new_solution = env.swap();
  println!("\x1b[31mSolution after swap:\x1b[0m");
  println!("{}", new_solution);
  println!("\x1b[31mDiversity:\x1b[0m {}", new_solution.calculate_diversity());
}
