use crate::mdp_problem::MDPProblem;
use crate::points::{Point, Point2d, Point3d, PointType};

pub struct MDPSolution {
  mdp_problem: MDPProblem,
  solution: Vec<PointType>
}

impl MDPSolution {
  pub fn new(mdp_problem: MDPProblem) -> Self {
    MDPSolution { mdp_problem, solution: vec![] }
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
    self.solution.push(point)
  }
  pub fn centroids(&self) -> PointType {
    let mut values : Vec<f32> = vec![];
    for i in 0..self.mdp_problem().points() as usize {
      let mut sum = 0.0;
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
    let mut sum = 0.0;
    for i in 0..self.solution.len() {
      for j in 0..self.solution.len() {
        if i != j {
          sum += self.solution[i].distance_euclidean(&self.solution[j]);
        }
      }
    }
    sum / (self.solution.len() * (self.solution.len() - 1)) as f32
  }
}

impl std::fmt::Display for MDPSolution {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "MDPSolution: solution = [")?;
    for point in &self.solution {
      writeln!(f, "{}", point)?;
    }
    write!(f, "]")
  }
}