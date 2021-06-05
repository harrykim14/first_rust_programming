use front_of_house::{hosting, serving};

fn serve_order() {}

mod back_of_house {
    fn cook_order() {}

    // 열거자를 public처리하면 모든 열것값도 public 처리 된다
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
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    // 절대 경로
    meal.toast = String::from("밀빵");
    // meal.seasonal_fruit;
    crate::front_of_house::hosting::add_to_waitlist(); // module `hosting` is private

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
    // 아무런 문제 없이 열것값을 사용할 수 있다
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
}
