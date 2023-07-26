fn main() {
    let width = 30;
    let height = 40;

    println!("area of rectangle is {}", area(width, height));

    let rect1 = (30, 50);

    println!("area of rectangle is {}", area_with_dimension(rect1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_dimension(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
