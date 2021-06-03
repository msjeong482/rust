use std::io;

fn main() {
    let mut input = String::new();
    let mut pig_latin = String::from("");

    io::stdin().read_line(&mut input)
        .expect("can not read line");
    input = input.trim().to_string();

    let first_word = input.as_bytes()[0];

    if first_word == b'a' || first_word == b'e' || first_word == b'i' || first_word == b'o' || first_word == b'u' {
        pig_latin = input + "-hay"; 
    } else {
        pig_latin = input[1..].to_string() + "-" + &input[..0] + "ay";
    }

    println!("pig latin:{}", pig_latin)
}
