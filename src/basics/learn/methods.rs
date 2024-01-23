fn run() {
    let width = 30;
    let height = 40;

    println!("area of rectangle is {}", area(width, height));

    let rect1 = (30, 50);

    println!("area of rectangle is {}", area_with_dimension(rect1));

    let scale = 3;
    let rect2 = Rectangle {
        width: dbg!(10 * scale),
        height: 60,
    };

    println!(
        "area of rectangle {:#?} is {}",
        rect2,
        area_with_rect(&rect2)
    );

    dbg!(&rect2);

    println!("area of rectangle {:#?} is {}", rect2, rect2.area());

    let square = Rectangle::square(32);
    println!("The area of square {}", square.area())
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_dimension(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_with_rect(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
