#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer_point = Point { x: 3, y: 10 };
    println!("{:#?}", integer_point);

    let float_point = Point { x: 2.2, y: 5.8 };
    println!("{:#?}", float_point);
    println!("{:#?}", float_point.distance_from_origin());

    let hybrid_point = Point { x: 5, y: 9.8 };
    println!("{:#?}", hybrid_point);
    println!("{:#?}", hybrid_point.x());
    println!("{:#?}", hybrid_point.y());

    let string_point = Point { x: "hello", y: 'c' };
    let mixed_up_point = string_point.mixup(float_point);
    println!("{:#?}", mixed_up_point);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
