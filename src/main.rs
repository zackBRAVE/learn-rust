use crate::garden::vegetables;

pub mod garden;

fn main() {
    let plant = vegetables::Asparagus {};
    println!("I'm growing {:?}!", plant);
}
