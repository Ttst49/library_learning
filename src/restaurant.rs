pub mod living_room {
    pub mod entrance {
        pub fn add_to_waitlist() {}

        fn bring_to_table() {}
    }

    pub mod service {
        fn take_order() {}

        pub fn give_order(){}

        fn cash_in() {}
    }

    pub mod kitchen{

        pub enum Appetizer{
            Soup,
            Salad,
        }

        pub struct Breakfast{
            pub grilled_toast: String,
            season_fruit: String
        }

        impl Breakfast {
            pub fn in_summer(grilled_toast: &str)->Breakfast{
                Breakfast{
                    grilled_toast: String::from(grilled_toast),
                    season_fruit: String::from("peaches"),
                }
            }
        }

        fn cook_order(){}

        fn correct_wrong_order(){
            cook_order();
            super::service::give_order();
        }
    }
}
