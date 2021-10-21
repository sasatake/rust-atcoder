use proconio::input;

fn main() {
  input! {
    n: usize,
    d: [i32; n],
  }
  let mut kagami_mochi = Vec::<i32>::new();
  for num in &d {
    match kagami_mochi.iter().find(|i| i == &num) {
      None => kagami_mochi.push(*num),
      _ => (),
    }
  }
  println!("{}", kagami_mochi.len());
}
