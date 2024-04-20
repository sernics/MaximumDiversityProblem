use crate::mdp_solution::MDPSolution;
use crate::mdp::mdp;
use crate::mdp_problem::MDPProblem;
use crate::points::Point;

pub struct MDPGreedy {
  mdp_solution: MDPSolution,
  size_m: u8
}

impl mdp for MDPGreedy {
  fn new(problem: MDPProblem, size_m: u8) -> Self {
    MDPGreedy { mdp_solution: MDPSolution::new(problem), size_m }
  }
  fn execute(&mut self) -> &MDPSolution {
    self.mdp_solution.insert(self.mdp_solution.mdp_problem().initial_point().copy());
    return &self.mdp_solution
  }
}



