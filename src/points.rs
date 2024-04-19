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