use crate::mdp_problem::MDPProblem;
use crate::points::{Point, Point2d, Point3d, PointType};
use std::ops::Index;
use std::usize;

#[derive(Clone)]
pub struct MDPSolution {
  mdp_problem: MDPProblem,
  solution: Vec<PointType>,
  diversity: f32
}

impl MDPSolution {
  pub fn new(mdp_problem: MDPProblem) -> Self {
    MDPSolution { mdp_problem, solution: vec![], diversity: 0.0 }
  }
  pub fn len(&self) -> usize {
    self.solution.len()
  }
  pub fn contains(&self, point: &PointType) -> bool {
    self.solution.contains(point)
  }
  pub fn mdp_problem(&self) -> &MDPProblem {
    &self.mdp_problem
  }
  pub fn insert(&mut self, point: PointType) {
    self.solution.push(point.clone());
    for i in 0..self.solution.len() {
      self.diversity += self.solution[i].distance_euclidean(&point);
    }
  }
  pub fn insert_at(&mut self, point: PointType, index: usize) {
    self.solution.insert(index, point.clone());
    for i in 0..self.solution.len() {
      self.diversity += self.solution[i].distance_euclidean(&point);
    }
  }
  pub fn drop(&mut self, index: usize) {
    let point = self.solution[index].clone();
    self.solution.remove(index);
    for i in 0..self.solution.len() {
      self.diversity -= self.solution[i].distance_euclidean(&point);
    }
  }
  pub fn centroids(&self) -> PointType {
    let mut values: Vec<f32> = vec![];
    for i in 0..self.mdp_problem().points() as usize {
      let mut sum: f32 = 0.0;
      for point in &self.solution {
        sum += point[i];
      }
      values.push(sum / self.solution.len() as f32);
    }
    match self.mdp_problem().initial_point() {
      PointType::Point2d(_) => PointType::Point2d(Point2d::new(values)),
      PointType::Point3d(_) => PointType::Point3d(Point3d::new(values)),
    }
  }
  pub fn calculate_diversity(&self) -> f32 {
    return self.diversity;
  }
}

impl PartialEq for MDPSolution {
  fn eq(&self, other: &Self) -> bool {
    self.diversity == other.diversity
  }
}

impl PartialOrd for MDPSolution {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.diversity.partial_cmp(&other.diversity).unwrap())
  }
}

impl Ord for MDPSolution {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.diversity.partial_cmp(&other.diversity).unwrap().reverse()
  }
}

impl Eq for MDPSolution {}

impl std::fmt::Display for MDPSolution {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "MDPSolution: solution = [")?;
    for point in &self.solution {
      writeln!(f, "{}", point)?;
    }
    write!(f, "]")
  }
}

impl Index <usize> for MDPSolution {
  type Output = PointType;
  fn index(&self, index: usize) -> &PointType {
    &self.solution[index]
  }
}