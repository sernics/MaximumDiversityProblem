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
  pub fn points(&self) -> u8 {
    self.points
  }
  pub fn states(&self) -> &Vec<PointType> {
    &self.states
  }
  pub fn initial_point(&self) -> &PointType {
    self.states.iter().max_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap()).unwrap()
  }
  pub fn next_point(&self, state: &PointType) -> PointType {
    self.states.iter().max_by(|a, b| state.distance_euclidean(a).partial_cmp(&state.distance_euclidean(b)).unwrap()).unwrap().copy()
  }
  pub fn select_k_next_points(&self, state: &PointType, k: u8) -> Vec<PointType> {
    // Seleccionar los k puntos más lejanos a state difentes entre sí
    let mut selected_points: Vec<PointType> = vec![];
    for _i in 0..k as usize {
      let mut max_distance = 0.0;
      let mut selected_point = self.states[0].copy();
      for point in &self.states {
        if !selected_points.contains(point) && state.distance_euclidean(point) > max_distance {
          max_distance = state.distance_euclidean(point);
          selected_point = point.copy();
        }
      }
      selected_points.push(selected_point);
    }
    // Print selected_points
    for point in &selected_points {
      println!("{}", point);
    }
    selected_points
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

impl Clone for MDPProblem {
  fn clone(&self) -> Self {
    MDPProblem { states: self.states.clone(), points: self.points }
  }
}