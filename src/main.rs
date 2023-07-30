use crate::garden::vegetables::Asparagus;

pub mod garden;
pub mod guess_game;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    guess_game::guessing_game();
}
