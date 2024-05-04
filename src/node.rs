use crate::mdp_solution::MDPSolution;

pub struct Node {
  solution: MDPSolution,
  upper_bound: f32,
  m: u8
}

impl Node {
  pub fn new(solution: MDPSolution, m: u8) -> Self {
    let upper_bound = solution.calculate_diversity();
    Node { solution, upper_bound, m }
  }
  pub fn solution(&self) -> &MDPSolution {
    &self.solution
  }
  pub fn upper_bound(&self) -> f32 {
    self.upper_bound
  }
  pub fn m(&self) -> u8 {
    self.m
  }
}