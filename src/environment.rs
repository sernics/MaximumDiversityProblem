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
    for i in 0..self.solution.len() {
      for j in 0..self.mdp_problem.states().len() as usize {
        let actual_diversity = self.solution.calculate_diversity();
        let drop_point = self.solution[i].clone();
        self.solution.drop(i);
        self.solution.insert_at(self.mdp_problem.states()[j].clone(), i);
        let new_diversity = self.solution.calculate_diversity();
        if new_diversity < actual_diversity {
          self.solution.drop(self.solution.len() - 1);
          self.solution.insert_at(drop_point, i);
        }
      }
    }
    return self.solution.clone();
  }
  pub fn generate_neighbours(&mut self) -> Vec<MDPSolution> {
    let mut neighbours: Vec<MDPSolution> = vec![];
    for i in 0..self.solution.len() {
      for j in 0..self.mdp_problem.states().len() as usize {
        let actual_diversity = self.solution.calculate_diversity();
        let drop_point = self.solution[i].clone();
        self.solution.drop(i);
        self.solution.insert_at(self.mdp_problem.states()[j].clone(), i);
        let new_diversity = self.solution.calculate_diversity();
        if new_diversity < actual_diversity {
          self.solution.drop(self.solution.len() - 1);
          self.solution.insert_at(drop_point, i);
        } else {
          neighbours.push(self.solution.clone());
        }
      }
    }
    return neighbours;
  }
}