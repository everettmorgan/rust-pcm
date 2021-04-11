mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist () {}

        fn seat_at_table () {}
    }

    mod serving {
        fn take_order () {}

        fn serve_order () {}

        fn take_payment () {}
    }
}

fn serve_order () {}

mod back_of_house {
    #[derive (Debug)]
    pub enum Appetizers {
        Soup, Salad
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // associated method
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from (toast),
                seasonal_fruit: String::from ("apples"),
            }
        }
    }

    fn fix_incorrect_order () {
        cook_order ();
        // super refers to this module (the global context)
        super::serve_order ();
    }

    fn cook_order () {}
}

// Adding use and a path in a scope is similar to
// creating a symbolic link in the filesystem.

// absolute path
use front_of_house::hosting;

// relative path
// use self::front_of_house::hosting;

pub fn eat_at_restaurant () {
    // verbose
    front_of_house::hosting::add_to_waitlist();

    // we can use `use` to bring front_of_house::hosting
    // into the scope of eat_at_restaurant
    hosting::add_to_waitlist();

    let app1 = back_of_house::Appetizers::Salad;
    let app2 = back_of_house::Appetizers::Soup;

    let mut meal = back_of_house::Breakfast::summer ("white");
    meal.toast = String::from ("wheat"); // change our minds

    println! ("I'd like {} toast please!", meal.toast);
    println! ("I'd like the {:?} appetizer please!", app1);
    println! ("I'd like the {:?} appetizer please!", app2);
}