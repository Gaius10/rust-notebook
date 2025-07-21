#![allow(unused_variables)]
#![allow(dead_code)]

// Private, so neither this or it's children can be seen out of
// this file.
mod front_of_house {
    // If this wouldn't public, it wouldn't be seen by
    // eat_at_restaurant function.
    pub mod hosting {
        // Same here
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast...
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Changing our mind about what to eat...
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    // Won't compile because it's private:
    // meal.seasonal_fruit = String::from("blueberries");

    // Public enums...
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// The super keyword:
fn deliver_order() {}

mod back_of_house {
    // Child nodes have access to all of its parents.
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // Public structs:
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // This can't be seen from outside
    }

    impl Breakfast {
        //
        // Obs.: note that, as Breakfast has private attributes,
        // it is not possible to create a instance of it without a
        // constructor function like this.
        //
        // In contrast, if an enum is public, so are all of
        // its variants.
        //
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                // Comming from outside:
                toast: String::from(toast),
                // Being privately defined by the "chef":
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}





