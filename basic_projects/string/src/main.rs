fn main() {

    //let mut s = String::new();

    let data = "initial string value";
    //let s = data.to_string();
    let mut s = String::from("initial string value ");
    s.push_str("bar");

    println!("{}", s);

    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello ");
    let s2 = String::from("World!");
    let s1 = s1 + &s2;
    println!("{}", s1);
    println!("{}", s2);
    
    let s3 = String::from("tic");
    let s4 = String::from("tac");
    let s5 = String::from("toe");
    let s6 = format!("{}-{}-{}", s3, s4, s5);
    println!("{}", s3);
    println!("{}", s4);
    println!("{}", s5);
    println!("{}", s6);

    let hello = "안녕하세여";

    let s7 = &hello[0..3];
    msp(&s7);

    for c in "안녕하세요".chars() {
        println!("{}", c);
    }
    for b in "안녕하세요".bytes() {
        println!("{}", b);
    }
}

fn msp(s: &str) {
    println!("{}", s);
}
