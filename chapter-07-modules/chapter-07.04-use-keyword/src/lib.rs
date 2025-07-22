#![allow(unused_variables)]
#![allow(dead_code)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Use is only valid in its scope, so this doesn't compile
// use crate::front_of_house::hosting;

mod customer {
    // This works
    pub use crate::front_of_house::hosting as abc;
    // `pub` is re-exporting the module.

    pub fn eat_at_restaurant() {
        abc::add_to_waitlist();
    }
}

