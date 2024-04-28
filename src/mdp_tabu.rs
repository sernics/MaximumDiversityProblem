use crate::mdp_solution::MDPSolution;
use crate::mdp::MDP;
use crate::mdp_problem::MDPProblem;

pub struct MDPTabu {
  mdp_solution: MDPSolution,
  size_m: u8,
  tabu_list: Vec<MDPSolution>
}

impl MDP for MDPTabu {  
  fn new(problem: MDPProblem, size_m: u8) -> Self {
    MDPTabu { mdp_solution: MDPSolution::new(problem), size_m, tabu_list: vec![] }
  }
  fn execute(&mut self) -> &MDPSolution {
    self.mdp_solution.insert(self.mdp_solution.mdp_problem().initial_point().clone());
    while self.mdp_solution.len() < self.size_m as usize {
      let centroid = self.mdp_solution.centroids();
      let selected = self.mdp_solution.mdp_problem().next_point(&centroid);
      if !self.mdp_solution.contains(&selected) {
        self.mdp_solution.insert(selected.clone());
      } else {
        let mut max_diversity = 0.0;
        let mut selected = self.mdp_solution.mdp_problem().initial_point().clone();
        for point in self.mdp_solution.mdp_problem().states() {
          if !self.mdp_solution.contains(point) {
            let mut actual_solution = self.mdp_solution.clone();
            actual_solution.insert(point.clone());
            if actual_solution.calculate_diversity() > max_diversity {
              max_diversity = actual_solution.calculate_diversity();
              selected = point.clone();
            }
          }
        }
        self.mdp_solution.insert(selected);
      }
      self.tabu_list.push(self.mdp_solution.clone());
    }
    return &self.mdp_solution;
  }
}