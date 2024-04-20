use crate::mdp_problem::MDPProblem;
use crate::mdp_solution::MDPSolution;

pub trait MDP {
  fn new(problem: MDPProblem, size_m: u8) -> Self;
  // fn states(&self) -> &Vec<PointType>;
  // fn points(&self) -> u8;
  fn execute(&mut self) -> &MDPSolution;
}