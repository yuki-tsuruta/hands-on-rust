#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        // 長方形の面積は、{}平方ピクセル。
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        // 長方形の面積は、{}平方ピクセル。
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        // 長方形の面積は、{}平方ピクセル。
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        // 長方形の面積は、{}平方ピクセル。
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );

    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect5 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect6 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect4 hold rect5? {}", rect4.can_hold(&rect5));

    let rect7 = Rectangle::square(3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
