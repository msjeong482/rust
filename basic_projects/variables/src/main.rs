const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = MAX_POINTS;
    println!("x의 값: {}", x);
    let x = x + 1;
    println!("x의 값: {}", x);
    let x = x * 2;
    println!("x의 값: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces : {}", spaces);
}

