use crate::mdp_solution::MDPSolution;
use crate::mdp::MDP;
use crate::mdp_problem::MDPProblem;
use crate::points::{Point2d, Point3d, PointType, Point};

use rand::seq::SliceRandom;

pub struct MDPGrasp {
  mdp_solution: MDPSolution,
  size_m: u8
}

impl MDP for MDPGrasp {
  fn new(problem: MDPProblem, size_m: u8) -> Self {
    MDPGrasp { mdp_solution: MDPSolution::new(problem), size_m }
  }
  fn execute(&mut self) -> &MDPSolution {
    const K_VALUE : u8 = 3;
    let initial_centroid = match self.mdp_solution.mdp_problem().initial_point() {
      PointType::Point2d(_) => PointType::Point2d(Point2d::new(vec![0.0, 0.0])),
      PointType::Point3d(_) => PointType::Point3d(Point3d::new(vec![0.0, 0.0, 0.0])),
    };
    let initial_points = self.mdp_solution.mdp_problem().select_k_next_points(&initial_centroid, K_VALUE);
    self.mdp_solution.insert(initial_points.choose(&mut rand::thread_rng()).unwrap().clone());
    while self.mdp_solution.len() < self.size_m as usize {
      let centroid = self.mdp_solution.centroids();
      let k_values = self.mdp_solution.mdp_problem().select_k_next_points(&centroid, K_VALUE);
      let selected = k_values.choose(&mut rand::thread_rng()).unwrap();
      if !self.mdp_solution.contains(selected) {
        self.mdp_solution.insert(selected.copy());
      }
    }
    return &self.mdp_solution;
  }
}