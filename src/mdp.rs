pub trait mdp {
  fn new(states: Vec<PointType>, points: u8) -> Self;
  // fn states(&self) -> &Vec<PointType>;
  // fn points(&self) -> u8;
  fn centroid(&self) -> PointType;
}