struct Point {
    x: i32,
    y: i32,
}

fn main() {

    let p = Point {x: 0, y: 7};
    let Point {x: a, y: b} = p;
    println!("a:{}, b:{}", a, b);
    let p1 = Point {x: 0, y: 7};
    let Point {x, y} = p1;
    println!("x:{}, y:{}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("11"),
        3 ..= 5 => println!("33"),
        _ => println!("00"),
    }

    let c = 'c';
    match c {
        'a' ..= 'j' => println!("ascii a - j"),
        'k' ..= 'z' => println!("ascii k - z"),
        _ => println!("etc .."),
    }

    let q = Some(4);
    match q {
        Some(x) if x < 5 => println!("x:{}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let y = Some(5);
    let z = 10;
    match y {
        Some(50) => println!("50"),
        Some(z) => println!("equal, z = {:?}", z),
        _ => println!("not equal, y = {:?}", y),
    }

    println!("result: y = {:?}, z = {:?}", y, z);

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("the favorite color is {}", color);
    } else if is_tuesday {
        println!("the color of tuesday is green!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("age > 30");
        } else {
            println!("age <= 30");
        }
    } else {
        println!("default");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("the value of index {}: {}", index, value);
    }

    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current position: {}, {}", x, y);
}
