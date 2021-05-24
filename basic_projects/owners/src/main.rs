fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);
    println!("{}, world!", s2);

    let x = 5;
    let y = x;
    println!("x: {}", x);
    println!("y: {}", y);

    let s = String::from("hello");
    takes_ownership(s);

    let x2 = 2;
    makes_copy(x2);

    let sc = String::from("hello");
    let len = calculate_length(&sc);
    println!("sc: {}, len: {}", sc, len);

    let mut msc = String::from("hello");
    change_string(&mut msc);
    println!("msc: {}", msc);

    let mut mscc = String::from("hello");

    {
        let _r1 = &mut mscc;
    }

    let _r2 = &mut mscc;

    let mut s3 = String::from("hello world");
    let s3_i = first_word(&s3);
    println!("s3_i:{}", s3_i);

//    s3.clear();
    println!("s3:{}", s3_i);

    let my_string = "haha hoho";
    let my_string2 = first_word(&my_string[..]);
    println!("my_string2:{}", my_string2);

    let s4 = String::from("hello world");
    let hello = &s4[0..5];
    let world = &s4[6..11];
    println!("hello: {}", hello);
    println!("world: {}", world);

    let m = String::from("minseong");
    let mut mm = m;
    mm.push_str("dd");
    println!("mm:{}", mm);

    let minseong_array = [1, 2, 3, 4, 5];
    let minseong_slice = &minseong_array[1..3];
    for (i, &item) in minseong_slice.iter().enumerate() {
        println!("value[{}]:{}", i, item);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
