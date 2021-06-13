
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("drop the CustomSmartPointer data:{}", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x1 = 5;
    let y1 = Box::new(x1);
    assert_eq!(5, x1);
    assert_eq!(5, *y1);

    let myx = 5;
    let myy = MyBox::new(myx);
    assert_eq!(5, myx);
    assert_eq!(5, *myy); // *(myy.deref())

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // (&(*m)[..]), deref coercion

    let c = CustomSmartPointer { data: String::from("my data") };
    let d = CustomSmartPointer { data: String::from("other data") };
    println!("CustomSmartPointer created");
    drop(c);
    println!("drop the CustomSmartPointer before end of main");

}

fn hello(name: &str) {
    println!("Hello {}!", name);
}
