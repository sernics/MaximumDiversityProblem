mod points;
mod parse;
mod mdp_problem;

fn main() {
  let path = std::path::PathBuf::from(
    std::env::args().nth(1).expect("No file path provided"));
  let points = parse::parse_file(&path);
}
