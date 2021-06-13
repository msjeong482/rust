
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5,
        Rc::new(Cons(10,
            Rc::new(Nil)))));

    println!("the counter after createing a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("the counter after createing b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("the counter after createing c = {}", Rc::strong_count(&a));
    }

    println!("the counter after drop c = {}", Rc::strong_count(&a));
}
