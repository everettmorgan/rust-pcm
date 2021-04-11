#[derive (Debug)]
pub enum Appetizers {
    Soup, Salad
}

#[derive (Debug)]
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

use crate::front_of_house::serving;

fn fix_incorrect_order () {
    cook_order ();
    serving::serve_order ();
}

fn cook_order () {}