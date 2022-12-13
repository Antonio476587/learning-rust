// Nested paths
pub use std::{cmp::Ordering, collections};
// Self nested path, it btings io and io::Write
use std::io::{self, Write};
// Glob operator
use std::collections::*;

mod front_of_house;

fn deliver_order() {}

mod back_of_house;

pub use crate::front_of_house::hosting;

mod costumer {
    // Shorting with 'use'
    use super::hosting as h;

    pub fn eat_at_restaurant() {
        h::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast
        let mut meal = super::back_of_house::breakfast::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
    }
}