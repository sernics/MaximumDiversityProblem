use crate::points::PointType;

pub struct MDPProblem {
  states: Vec<PointType>,
  points: u8
}

impl MDPProblem {
  pub fn new(states: Vec<PointType>, points: u8) -> Self {
    MDPProblem { states, points }
  }
}