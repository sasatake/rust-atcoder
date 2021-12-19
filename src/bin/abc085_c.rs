use proconio::input;

fn main() {
  input! {
      n: i32,
      y: i32,
  }
  let mut result = [-1, -1, -1];
  let max_paper_count_10000 = y / 10000;

  for paper_count_10000 in 0..max_paper_count_10000 + 1 {
    let remaining_paper_count = n - paper_count_10000;

    if remaining_paper_count == 0 {
      let sum = 10000 * paper_count_10000 + 5000 * 0 + 1000 * 0;
      if sum == y {
        result = [paper_count_10000, 0, 0]
      }
    }

    for paper_count_5000 in 0..remaining_paper_count {
      let paper_count_1000 = remaining_paper_count - paper_count_5000;
      let sum = 10000 * paper_count_10000 + 5000 * paper_count_5000 + 1000 * paper_count_1000;
      if sum == y {
        result = [paper_count_10000, paper_count_5000, paper_count_1000]
      }
    }
  }
  println!("{} {} {}", result[0], result[1], result[2]);
}
