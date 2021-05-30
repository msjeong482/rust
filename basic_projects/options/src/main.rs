
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 7u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value2 = Some(0u8);
    match some_u8_value2 {
        Some(0) => println!("zero"),
        _ => (),
    }

    if let Some(0) = some_u8_value2 {
        println!("zero");
    } else {
        println!("hi");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
