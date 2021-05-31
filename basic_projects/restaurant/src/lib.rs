#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house;

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("pich"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

//use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;
use self::front_of_house::serving;

use std::fmt::Result;
use std::io::Result as IoResult;

use std::{io, cmp::Ordering};

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    meal.toast = String::from("밀빵");
    println!("{} 토스트로 주세요", meal.toast);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    serving::take_order();
}
