use crate::mdp_solution::MDPSolution;
use crate::mdp::MDP;
use crate::mdp_problem::MDPProblem;
use crate::mdp_grasp::MDPGrasp;
use crate::environment::Environment;

use std::collections::VecDeque;

pub struct MDPTabu {
  mdp_solution: MDPSolution,
  size_m: u8,
  tabu_list: VecDeque<MDPSolution>,
  max_iterations: u8
}

impl MDP for MDPTabu {  
  fn new(problem: MDPProblem, size_m: u8) -> Self {
    MDPTabu { mdp_solution: MDPSolution::new(problem), size_m, tabu_list: VecDeque::new(), max_iterations: 100 }
  }
  fn execute(&mut self) -> &MDPSolution {
    self.mdp_solution = MDPGrasp::new(self.mdp_solution.mdp_problem().clone(), self.size_m).execute().clone();
    let mut best_distance = self.mdp_solution.calculate_diversity();
    let mut current_solution = self.mdp_solution.clone();
    let mut iterations = 0;
    while iterations < self.max_iterations {
      let candidate_solution = generate_candidate_solution(current_solution.clone());
      let candidate_distance = candidate_solution.calculate_diversity();
      if candidate_distance < best_distance {
        self.mdp_solution = candidate_solution.clone();
        best_distance = candidate_solution.calculate_diversity();
      }
      self.tabu_list.push_back(candidate_solution.clone());
      if self.tabu_list.len() > self.size_m as usize {
        self.tabu_list.pop_front();
      }
      current_solution = candidate_solution.clone();
      iterations += 1;
    }
    return &self.mdp_solution;
  }
}

fn generate_candidate_solution(current_solution: MDPSolution) -> MDPSolution {
  let mut env = Environment::new(current_solution.mdp_problem().clone(), current_solution.clone());
  let neighbours: Vec<MDPSolution> = env.generate_neighbours();
  let mut candidate_solution = current_solution.clone();
  for neighbour in neighbours {
    if neighbour.calculate_diversity() > candidate_solution.calculate_diversity() {
      candidate_solution = neighbour;
    }
  }
  candidate_solution
}