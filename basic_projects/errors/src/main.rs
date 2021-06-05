
use std::fs::File;
// use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;
use std::net::IpAddr;

fn main() {
// fn main() -> Result<(), Box<dyn Error>> {
    // let f = File::open("hello.txt").expect("Failed to open the file.");
//    let f = File::open("hello.txt").unwrap();
/*
    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create the file: {:?}", e),
            },
            other_error => panic!("Failed to open the file: {:?}", other_error),
        }
    };
*/

//    read_username_from_file().expect("Failed to read the file");
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("home:{:?}", home);
    read_username_from_file4().expect("Failed to read the file");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hellof.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hellof2.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s) 
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hellof3.txt")?.read_to_string(&mut s)?;

    Ok(s) 
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello4.txt")
}
