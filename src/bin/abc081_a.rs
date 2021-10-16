use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
      s1s2s3: Chars,
  }
  println!("{}", s1s2s3.iter().filter(|&n| *n == '1').count());
}
