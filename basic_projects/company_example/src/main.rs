pub use company_example::department;

fn main() {
    println!("Hello, world!");
    let engineering = department::engineering::add_to_department(String::from("hi"));
    let sales = department::sales::add_to_department(String::from("hi"));

    println!("engineering:{:?}", engineering);
    println!("sales:{:?}", sales);
}
