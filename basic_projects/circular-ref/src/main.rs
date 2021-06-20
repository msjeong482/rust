
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("initial count of a = {}", Rc::strong_count(&a));
    println!("the next item of a = {:?}", a.tail());

    let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));

    println!("count of a after create b = {}", Rc::strong_count(&a));
    println!("initial count of b = {}", Rc::strong_count(&b));
    println!("the next item of b = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("count of b after change a = {}", Rc::strong_count(&b));
    println!("count of a after change a = {}", Rc::strong_count(&a));
//    println!("the next item of a = {:?}", a.tail());
}
