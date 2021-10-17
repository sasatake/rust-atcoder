use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [i32; n],
  }
  let mut count: u8 = 0;
  let mut temp_a = a;
  while temp_a.iter().all(|i| i % 2 == 0) {
    temp_a = temp_a.iter().map(|i| i / 2).collect();
    count += 1;
  }
  println!("{:?}", count);
}
