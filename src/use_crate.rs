pub mod garden;

use crate::garden::vegetables;

fn use_crate() {
    let plant = vegetables::Asparagus {};
    println!("I'm growing {:?}!", plant);
}
