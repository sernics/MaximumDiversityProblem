use crate::mdp_solution::MDPSolution;
use crate::mdp::MDP;
use crate::mdp_problem::MDPProblem;
use crate::points::Point;

pub struct MDPGreedy {
  mdp_solution: MDPSolution,
  size_m: u8
}

impl MDP for MDPGreedy {
  fn new(problem: MDPProblem, size_m: u8) -> Self {
    MDPGreedy { mdp_solution: MDPSolution::new(problem), size_m }
  }
  fn execute(&mut self) -> &MDPSolution {
    let initial_point = self.mdp_solution.mdp_problem().initial_point();
    self.mdp_solution.insert(initial_point.copy());
    while self.mdp_solution.len() < self.size_m as usize {
      let centroid = self.mdp_solution.centroids();
      let selected = self.mdp_solution.mdp_problem().next_point(&centroid);
      self.mdp_solution.insert(selected.copy());
    }
    return &self.mdp_solution
  }
}



