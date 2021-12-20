use proconio::input;

fn main() {
  input! {
      s: String,
  }

  let string_part_list = ["dream", "dreamer", "erase", "eraser"];

  if s.len() < 5 {
    println!("NO");
    return;
  }

  let mut s_reverse: String = s.chars().rev().collect();

  loop {
    let mut is_not_matched = false;
    for string_part in string_part_list
      .iter()
      .map(|s| s.chars().rev().collect::<String>())
    {
      if s_reverse.starts_with(&string_part) {
        s_reverse = s_reverse.replacen(&string_part, "", 1);
        is_not_matched = true;
        break;
      }
    }
    if !is_not_matched {
      break;
    }
  }

  if s_reverse.len() == 0 {
    println!("YES");
  } else {
    println!("NO");
  }
}
