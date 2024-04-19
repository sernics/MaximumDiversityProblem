mod points;

use points::Point;

fn main() {
  let vec2 : Vec<f32> = vec![1.0, 2.0];
  let vec3 : Vec<f32> = vec![1.0, 2.0, 3.0];
  let point2d = points::Point2d::new(vec2);
  let point3d = points::Point3d::new(vec3);
  println!("Point2d: {}, {}", point2d[0], point2d[1]);
  println!("Point3d: {}, {}, {}", point3d[0], point3d[1], point3d[2]);
}
