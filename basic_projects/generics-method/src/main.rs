
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn y(&self) -> &f32 {
        &self.y
    }
}

struct Test<T, U> {
    x: T,
    y: U,
}

impl<T, U> Test<T, U> {
    fn mixup<V, W> (self, other: Test<V, W>) -> Test<T, W> {
        Test {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    let fp = Point { x: 5.0, y: 10.1 };

    println!("p.x = {}", p.x());
    println!("fp.y = {}", fp.y());

    let t1 = Test { x: 5, y: 10.4 };
    let t2 = Test { x: "Hello", y: 'c' };

    let t3 = t1.mixup(t2);
    println!("p3.x = {}, p3.y = {}", t3.x, t3.y);
}
