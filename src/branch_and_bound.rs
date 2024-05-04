use crate::{mdp_problem::MDPProblem, mdp_solution::MDPSolution, node::Node};
use crate::mdp_greedy::MDPGreedy;
use crate::mdp_grasp::MDPGrasp;
use crate::points::Point;
use crate::points::PointType;

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
    let mut actual_node = Node::new(self.initial_solution.clone(), 0);
    let mut previous: Vec<PointType> = Vec::new();
    self.branch_and_bound(actual_node, previous);
    self.solution.borrow().clone()
  }
  pub fn branch_and_bound(&self, actual_node: Node, previous: Vec<PointType>) {
    let mut actual_nodes: Vec<Node> = Vec::new();
    for i in 0..self.points.states().len() {
      // Si el punto ya fue visitado, no se vuelve a visitar
      if previous.contains(&self.points[i as usize]) {
        continue;
      }
      let mut point = self.initial_solution.clone();
      point.drop(actual_node.m() as usize);
      point.insert_at(self.points[i as usize].clone(), actual_node.m() as usize);
      println!("Point {}: {}", i, point);
      actual_nodes.push(Node::new(point, actual_node.m() + 1));
    }
    
    if self.estrategy != "deep" {
      actual_nodes.sort_by(|a, b| b.upper_bound().partial_cmp(&a.upper_bound()).unwrap());
    }

    // Criterio de parada
    if actual_node.m() == self.max_m {
      // Queremos maximizar la diversidad
      if actual_node.upper_bound() > self.lower_bound.get() {
        self.solution.replace(actual_node.solution().clone());
        self.lower_bound.set(actual_node.upper_bound());
      }
      return;
    }
    for node in actual_nodes {
      if node.upper_bound() > self.lower_bound.get() {
        let mut new_previous = previous.clone();
        new_previous.push(self.points[node.m() as usize].clone());
        self.branch_and_bound(node, new_previous);
      }
    }
  }
}