// Define some modules for a restaurant
// some things happen front of house
mod front_of_house {
    // like hosting and seating
    // hosting and its child functions are public, so
    // eat_at_restaurant() can access their code execution
    pub mod hosting {
        // hosts might have multiple responsibilities
        pub fn add_to_waitlist() {} // empty for demonstration

        // or seat customers at tables
        pub fn seat_at_table() {}
    }

    // or serving food
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// maybe the library includes functions that rely on submodules
pub fn eat_at_restaurant() {
    // we could reference another submodule function using an absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // or the relative path
    front_of_house::hosting::seat_at_table();
}
