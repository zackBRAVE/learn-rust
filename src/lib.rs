mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::delivery_order();
    }

    fn cook_order() {}
}

fn delivery_order() {}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();
    front_of_house::hosting::add_to_wait_list();
    hosting::add_to_wait_list();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    back_of_house::Appetizer::Salad;
}

mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_wait_list();
        super::hosting::add_to_wait_list();
        super::front_of_house::hosting::add_to_wait_list();
    }
}

use std::collections::*;
use std::io::{self, Write};
use std::{cmp::Ordering, collections::HashMap as hashmap};

fn use_hashmap() {
    let mut v = hashmap::new();
    let mut vh: HashMap<i32, i32> = HashMap::new();
    v.insert(1, 2);
    v.entry(1).or_insert(4);
    v.entry(3).or_insert(4);
}
