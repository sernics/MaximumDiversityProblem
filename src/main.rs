mod points;
mod parse;
mod mdp_problem;
mod mdp_solution;
mod mdp_greedy;
mod mdp_grasp;
mod mdp;
mod environment;
mod mdp_tabu;
mod node;
mod branch_and_bound;

use crate::mdp_problem::MDPProblem;
use crate::mdp::MDP;
use crate::mdp_solution::MDPSolution;
use crate::mdp_tabu::MDPTabu;
use crate::points::PointType;
use crate::branch_and_bound::BranchAndBound;

fn main() {
  let path = std::path::PathBuf::from(
    std::env::args().nth(1).expect("No file path provided"));
  let points: MDPProblem = parse::parse_file(&path);
  println!("\x1b[31mPoints:\x1b[0m");
  println!("{}", points);
  const SIZE_M: u8 = 4;
  let mut greedy = mdp_greedy::MDPGreedy::new(points.clone(), SIZE_M); // Clone the points variable
  let greedy_result = greedy.execute();
  println!("\x1b[31mGreedy:\x1b[0m");
  println!("{}", greedy_result);
  println!("\x1b[31mDiversity greedy:\x1b[0m {}", greedy_result.calculate_diversity());
  let mut grasp = mdp_grasp::MDPGrasp::new(points.clone(), SIZE_M);
  let grasp_result : &MDPSolution = grasp.execute();
  println!("\x1b[31mGrasp:\x1b[0m");
  println!("{}", grasp_result);
  println!("\x1b[31mDiversity grasp:\x1b[0m {}", grasp_result.calculate_diversity());
  let mut tabu = MDPTabu::new(points.clone(), SIZE_M);
  let result = tabu.execute();
  println!("\x1b[31mTabu:\x1b[0m");
  println!("{}", result);
  println!("\x1b[31mDiversity tabu:\x1b[0m {}", result.calculate_diversity());
  let mut branch_and_bound = BranchAndBound::new(greedy_result.clone(), SIZE_M, "deep".to_string());
  let branch_and_bound_result = branch_and_bound.execute();
  println!("\x1b[31mBranch and bound:\x1b[0m");
  println!("{}", branch_and_bound_result);
}
