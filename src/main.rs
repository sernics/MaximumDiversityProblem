mod points;
mod parse;
mod mdp_problem;

fn main() {
  let path = std::path::PathBuf::from(
    std::env::args().nth(1).expect("No file path provided"));
  let points = parse::parse_file(&path);
  println!("{}", points);
  let initial_point = points.initial_point();
  println!("Initial point: {}", initial_point);
}