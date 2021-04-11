mod front_of_house;
mod back_of_house;

pub use crate::{
    front_of_house::{hosting, serving},
    back_of_house::{cooking},
};

// Adding use and a path in a scope is similar to
// creating a symbolic link in the filesystem.

// absolute path with a new name using `as`
// use front_of_house::hosting as hosting;

// relative path
// use self::front_of_house::hosting;

// re-exporting names with `pub use`
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant () {
    // verbose
    hosting::add_to_waitlist ();
    serving::serve_order ();
    let breakfast = cooking::Breakfast::summer ("wheat");
    println! ("{:?}", breakfast);

    // // we can use `use` to bring front_of_house::hosting
    // // into the scope of eat_at_restaurant
    // hosting::add_to_waitlist();
    //
    // let app1 = back_of_house::Appetizers::Salad;
    // let app2 = back_of_house::Appetizers::Soup;
    //
    // let mut meal = back_of_house::Breakfast::summer ("white");
    // meal.toast = String::from ("wheat"); // change our minds
    //
    // println! ("I'd like {} toast please!", meal.toast);
    // println! ("I'd like the {:?} appetizer please!", app1);
    // println! ("I'd like the {:?} appetizer please!", app2);
}