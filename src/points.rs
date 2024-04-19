use std::ops::Index;

pub trait Point {
  fn new(coords: Vec<f32>) -> Self;
  fn dimensions(&self) -> usize;
}

pub struct Point2d {
  x: f32,
  y: f32
}

pub struct Point3d {
  x: f32,
  y: f32,
  z: f32
}

pub enum PointType {
  Point2d(Point2d),
  Point3d(Point3d)
}

impl Point for Point2d {
  fn new(coords: Vec<f32>) -> Self {
    Point2d { x: coords[0], y: coords[1] }
  }
  fn dimensions(&self) -> usize {
    2
  }
}

impl Index<usize> for Point2d {
  type Output = f32;
  fn index(&self, index: usize) -> &f32 {
    match index {
      0 => &self.x,
      1 => &self.y,
      _ => panic!("Index out of bounds")
    }
  }
}

impl Point for Point3d {
  fn new(coords: Vec<f32>) -> Self {
    Point3d { x: coords[0], y: coords[1], z: coords[2] }
  }
  fn dimensions(&self) -> usize {
    3
  }
}

impl Index<usize> for Point3d {
  type Output = f32;
  fn index(&self, index: usize) -> &f32 {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Index out of bounds")
    }
  }
}

impl Point for PointType {
  fn new(coords: Vec<f32>) -> Self {
    match coords.len() {
      2 => PointType::Point2d(Point2d::new(coords)),
      3 => PointType::Point3d(Point3d::new(coords)),
      _ => panic!("Invalid number of coordinates")
    }
  }
  fn dimensions(&self) -> usize {
    match self {
      PointType::Point2d(p) => p.dimensions(),
      PointType::Point3d(p) => p.dimensions()
    }
  }
}

impl std::fmt::Display for Point2d {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Point2d: x = {}, y = {}", self.x, self.y)
  }
}

impl std::fmt::Display for Point3d {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Point3d: x = {}, y = {}, z = {}", self.x, self.y, self.z)
  }
}

impl std::fmt::Display for PointType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      PointType::Point2d(p) => write!(f, "{}", p),
      PointType::Point3d(p) => write!(f, "{}", p)
    }
  }
}