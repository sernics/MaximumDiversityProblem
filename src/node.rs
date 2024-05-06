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

impl Eq for Node {}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.upper_bound == other.upper_bound
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.upper_bound.partial_cmp(&other.upper_bound)
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.upper_bound.partial_cmp(&other.upper_bound).unwrap()
  }
}

impl Clone for Node {
  fn clone(&self) -> Self {
    Node { solution: self.solution.clone(), upper_bound: self.upper_bound, m: self.m }
  }
}