#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "사각형의 면적: {} 제곱 픽셀",
        rect1.area()
    );

    println!("rect1: {:#?}", rect1);

    println!("rect1 은 rect2 를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1 은 rect3 를 포함하는가? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(20);
    println!("rect4: {:#?}", rect4);
}

