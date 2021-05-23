fn main() {
    another_function(5, 6);
    let x = five();
    println!("x의 값 : {}", x);
}

fn another_function(x: u32, y: u32) {
    println!("x의 값: {}", x);
    println!("y의 값: {}", y);
}

fn five() -> u32 {
    5
}
