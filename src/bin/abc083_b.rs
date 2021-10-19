use proconio::input;

fn get_digit_sum(n: i32) -> i32 {
  let mut digit_sum = 0;
  let mut current_value = n;
  while current_value > 0 {
    digit_sum = digit_sum + (current_value % 10);
    current_value = current_value / 10;
  }
  return digit_sum;
}

fn main() {
  input! {
      n: i32,
      a: i32,
      b: i32,
  }
  let mut digit_sum_sum: i32 = 0;
  for num in 1..n + 1 {
    let digit_sum: i32 = get_digit_sum(num);
    digit_sum_sum += if a <= digit_sum && digit_sum <= b {
      num
    } else {
      0
    };
  }
  println!("{}", digit_sum_sum);
}
