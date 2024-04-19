use std::path::PathBuf;

use crate::points::{Point2d, Point3d, PointType};
use crate::points::Point;

pub fn parse_file(path: &PathBuf) -> Vec<PointType> {
  let mut points: Vec<PointType> = Vec::new();
  let contents = std::fs::read_to_string(path).unwrap();
  let points_size = contents.lines().next().unwrap().parse::<i32>().unwrap();
  let atributes_size = contents.lines().nth(1).unwrap().parse::<i32>().unwrap();
  for line in contents.lines().skip(2).take(points_size as usize) {
    let coords: Vec<f32> = line.split_whitespace()
      .map(|s| 
        s.replace(",", ".")
        .parse::<f32>().unwrap())
      .collect();
    match atributes_size {
      2 => points.push(PointType::Point2d(Point2d::new(coords))),
      3 => points.push(PointType::Point3d(Point3d::new(coords))),
      _ => panic!("Invalid number of atributes")
    }
  }
  points
}