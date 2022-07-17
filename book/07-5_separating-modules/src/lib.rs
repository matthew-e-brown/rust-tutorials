#![allow(unused)]


fn serve_order() {}

mod front_of_house;
mod back_of_house;

pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    /*

    // Absolute
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative
    front_of_house::hosting::add_to_waitlist();

    */

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast, please!", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}