use crate::mdp_problem::MDPProblem;
use crate::mdp_solution::MDPSolution;

pub struct Environment {
  mdp_problem: MDPProblem,
  solution: MDPSolution
}

impl Environment {
  pub fn new(mdp_problem: MDPProblem, solution: MDPSolution) -> Self {
    Environment { mdp_problem, solution }
  }
  pub fn swap(&mut self) -> MDPSolution {
    let mut best_solution = self.solution.clone();
    for i in 0..best_solution.len() {
      for j in 0..self.mdp_problem.states().len() as usize {
        let mut actual_solution = best_solution.clone();
        let point = self.mdp_problem[j].clone();
        actual_solution.swap(point, i);
        if actual_solution.calculate_diversity() > best_solution.calculate_diversity() {
          best_solution = actual_solution.clone();
        }
      }
    }
    best_solution
  }
}