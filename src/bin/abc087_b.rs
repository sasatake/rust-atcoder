use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }
    const A: i32 = 500;
    const B: i32 = 100;
    const C: i32 = 50;
    let mut pattern_count: i32 = 0;
    for ai in 0..a + 1 {
        for bi in 0..b + 1 {
            for ci in 0..c + 1 {
                if ai * A + bi * B + ci * C == x {
                    pattern_count += 1;
                }
            }
        }
    }
    println!("{}", pattern_count);
}
