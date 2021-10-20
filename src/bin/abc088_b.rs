use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [i32; n],
  }
  let mut alice_card_list = Vec::<i32>::new();
  let mut bob_card_list = Vec::<i32>::new();
  let mut temp_a = a;
  temp_a.sort();
  temp_a.reverse();

  for (i, card) in temp_a.iter().enumerate() {
    if i % 2 == 0 {
      alice_card_list.push(*card);
    } else {
      bob_card_list.push(*card);
    }
  }
  println!(
    "{}",
    alice_card_list.iter().sum::<i32>() - bob_card_list.iter().sum::<i32>()
  );
}
