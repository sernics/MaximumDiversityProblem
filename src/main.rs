mod points;
mod parse;

use points::PointType;

fn main() {
  let path = std::path::PathBuf::from(
    std::env::args().nth(1).expect("No file path provided"));
  let points : Vec<PointType> = parse::parse_file(&path);
  println!("Points:");
  for point in points {
    match point {
      points::PointType::Point2d(p) => {
        println!("{}", p);
      },
      points::PointType::Point3d(p) => {
        println!("{}", p);
      }
    }
  }
}
