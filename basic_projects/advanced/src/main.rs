use std::ops::Add;

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("pilot");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("wizard");
    }
}

impl Human {
    fn fly(&self) {
        println!("human");
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
