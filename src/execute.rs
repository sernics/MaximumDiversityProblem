use std::path::PathBuf;

use crate::mdp::MDP;
use crate::mdp_greedy::MDPGreedy;
use crate::branch_and_bound::BranchAndBound;
use crate::parse::parse_file;

pub fn execute() {
  let start_path = "data/max_div_";
  let points_length = [15, 20, 30];
  let points_size = [2, 3];
  let m_size = [2, 3, 4, 5];
  let mut result_print = String::from("Problema, n, K, m, z, CPU\n");
  for p in points_length.iter() {
    for ps in points_size.iter() {
      let actual_path = start_path.to_string() + &p.to_string() + "_" + &ps.to_string() + ".txt";
      let path = PathBuf::from(actual_path.clone());
      let points = parse_file(&path);
      for m in m_size.iter() {
        let mut greedy = MDPGreedy::new(points.clone(), *m);
        let bb = BranchAndBound::new(greedy.execute().clone(), *m, "deep".to_string());
        let start = std::time::Instant::now();  
        let result = bb.execute();
        let duration = start.elapsed();
        let time = duration.as_secs_f64();
        result_print += &format!("{}, {}, {}, {}, {}, {}ms\n", actual_path, p, ps, m, result.calculate_diversity(), time);
      }
      result_print += "\n";
    }
  }
  let print_path = PathBuf::from("results/orden_greedy_bb.csv");
  std::fs::write(print_path, result_print).expect("Unable to write file");
}