use std::ops::Index;

pub trait Point {
  fn new(coords: Vec<f32>) -> Self;
  fn dimensions(&self) -> usize;
  fn copy(&self) -> Self;
  fn distance(&self) -> f32;
  fn distance_euclidean(&self, other: &Self) -> f32;
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
  fn distance(&self,) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
  fn distance_euclidean(&self, other: &Self) -> f32 {
    ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
  }
  fn copy(&self) -> Self {
    Point2d { x: self.x, y: self.y }
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
  fn distance(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
  }
  fn distance_euclidean(&self, other: &Self) -> f32 {
    ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
  }
  fn copy(&self) -> Self {
    Point3d { x: self.x, y: self.y, z: self.z }
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
  fn distance(&self) -> f32 {
    match self {
      PointType::Point2d(p) => p.distance(),
      PointType::Point3d(p) => p.distance()
    }
  }
  fn distance_euclidean(&self, other: &Self) -> f32 {
    match (self, other) {
      (PointType::Point2d(p1), PointType::Point2d(p2)) => p1.distance_euclidean(p2),
      (PointType::Point3d(p1), PointType::Point3d(p2)) => p1.distance_euclidean(p2),
      _ => panic!("Invalid number of coordinates")
    }
  }
  fn copy(&self) -> Self {
    match self {
      PointType::Point2d(p) => PointType::Point2d(p.copy()),
      PointType::Point3d(p) => PointType::Point3d(p.copy())
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

impl Index<usize> for PointType {
  type Output = f32;
  fn index(&self, index: usize) -> &f32 {
    match self {
      PointType::Point2d(p) => p.index(index),
      PointType::Point3d(p) => p.index(index)
    }
  }
}

impl PartialEq for PointType {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (PointType::Point2d(p1), PointType::Point2d(p2)) => p1.x == p2.x && p1.y == p2.y,
      (PointType::Point3d(p1), PointType::Point3d(p2)) => p1.x == p2.x && p1.y == p2.y && p1.z == p2.z,
      _ => false
    }
  }
}