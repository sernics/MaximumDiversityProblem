use crate::{mdp_problem::MDPProblem, mdp_solution::MDPSolution, node::Node};
use crate::points::{Point, PointType};

use std::cell::{Cell, RefCell};

pub struct BranchAndBound {
  lower_bound: Cell<f32>,
  points: MDPProblem,
  initial_solution: MDPSolution,
  solution: RefCell<MDPSolution>,
  max_m: u8,
  estrategy: String
}

impl BranchAndBound {
  pub fn new(solution: MDPSolution, size_m: u8, estrategy: String) -> Self {
    let inferior_limit = solution.calculate_diversity();
    BranchAndBound { 
      lower_bound: Cell::new(inferior_limit), 
      points: solution.mdp_problem().clone(), 
      initial_solution: solution.clone(),
      solution: RefCell::new(MDPSolution::new(solution.mdp_problem().clone())),
      max_m: size_m,
      estrategy
    }
  }

  pub fn execute(&self) -> MDPSolution {
    let actual_node = Node::new(self.initial_solution.clone(), 0);
    let previous: Vec<PointType> = Vec::new();
    self.branch_and_bound(actual_node, previous);
    self.solution.borrow().clone()
  }

  pub fn branch_and_bound(&self, actual_node: Node, previous: Vec<PointType>) {
    let actual_nodes: Vec<Node> = self.generate_actual_nodes(&actual_node, previous);

    self.update_lower_bound(&actual_node, &actual_nodes);

    if self.estrategy == "deep".to_string() {
      self.in_deep(actual_node, actual_nodes);
    } else {
      println!("Estrategia no implementada")
    }
  }
  
  pub fn generate_actual_nodes(&self, actual_node: &Node, previous: Vec<PointType>) -> Vec<Node> {
    let mut actual_nodes: Vec<Node> = Vec::new();
    for i in 0..self.points.states().len() {
      let mut actual_result = MDPSolution::new(self.points.clone());
      for point in previous.iter() {
        actual_result.insert(point.clone());
      }
      actual_result.insert(self.points.states()[i].clone());
      while actual_result.len() < self.max_m as usize {
        let mut max_distance = 0.0;
        let mut max_point : PointType = match self.points.states()[0] {
          PointType::Point2d(_) => PointType::Point2d(Point::new(vec![0.0, 0.0])),
          PointType::Point3d(_) => PointType::Point3d(Point::new(vec![0.0, 0.0, 0.0])),
        };
        for point in self.points.states() {
          if !actual_result.contains(point) {
            let mut distance = 0.0;
            for i in 0..actual_result.len() {
              distance += actual_result.get_solution()[i].distance_euclidean(point);
            }
            if distance > max_distance {
              max_distance = distance;
              max_point = point.clone();
            }
          }
        }
        actual_result.insert(max_point);  
      }
      if actual_result.calculate_diversity() > self.lower_bound.get() {
        let node = Node::new(actual_result, actual_node.m() + 1);
        actual_nodes.push(node);
      }
    }
    actual_nodes
  }

  pub fn update_lower_bound(&self, actual_node: &Node, actual_nodes: &Vec<Node>) {
    if actual_node.m() == self.max_m - 2 {
      if actual_nodes.len() > 0 {
        // Buscar el mejor nodo
        let best_diversity_node = actual_nodes.iter().max_by(|a, b| a.upper_bound().partial_cmp(&b.upper_bound()).unwrap()).unwrap();
        if best_diversity_node.upper_bound() > self.lower_bound.get() {
          self.lower_bound.set(best_diversity_node.upper_bound());
          self.solution.replace(best_diversity_node.solution().clone());
        }
      }
    }
  }

  pub fn in_deep(&self, actual_node: Node, actual_nodes: Vec<Node>) {
    for node in actual_nodes {
      let mut previous: Vec<PointType> = Vec::new();
      for i in 0..actual_node.m() + 1 {
        previous.push(node.solution().get_solution()[i as usize].clone());
      }
      self.branch_and_bound(node, previous);
    }
  }
}