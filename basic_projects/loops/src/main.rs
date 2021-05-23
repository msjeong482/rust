fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("발사!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("요소의 값: {}", element);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }

    println!("발사!");
}
