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
    self.mdp_solution.insert(self.mdp_solution.mdp_problem().initial_point().copy());
    while self.mdp_solution.len() < self.size_m as usize {
      let centroid = self.mdp_solution.centroids();
      let selected = self.mdp_solution.mdp_problem().next_point(&centroid);
      if !self.mdp_solution.contains(&selected) {
        self.mdp_solution.insert(selected.copy());
      } else {
        let mut max_diversity = 0.0;
        let mut selected = self.mdp_solution.mdp_problem().initial_point().copy();
        for point in self.mdp_solution.mdp_problem().states() {
          if !self.mdp_solution.contains(point) {
            let mut actual_solution = self.mdp_solution.clone();
            actual_solution.insert(point.copy());
            if actual_solution.calculate_diversity() > max_diversity {
              max_diversity = actual_solution.calculate_diversity();
              selected = point.copy();
            }
          }
        }
        self.mdp_solution.insert(selected);
      }
    }
    return &self.mdp_solution;
  }
}