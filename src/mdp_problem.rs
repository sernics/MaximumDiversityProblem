use std::ops::Index;

use crate::points::{Point, PointType};

pub struct MDPProblem {
  states: Vec<PointType>,
  points: u8
}

impl MDPProblem {
  pub fn new(states: Vec<PointType>, points: u8) -> Self {
    MDPProblem { states, points }
  }
  pub fn states(&self) -> &Vec<PointType> {
    &self.states
  }
  pub fn points(&self) -> u8 {
    self.points
  }
  pub fn initial_point(&self) -> &PointType {
    self.states.iter().max_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap()).unwrap()
  }
}

impl Index<usize> for MDPProblem {
  type Output = PointType;
  fn index(&self, index: usize) -> &PointType {
    &self.states[index]
  }
}

impl std::fmt::Display for MDPProblem {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "MDPProblem: states = [")?;
    for state in &self.states {
      writeln!(f, "{}", state)?;
    }
    write!(f, "], points = {}", self.points)
  }
}