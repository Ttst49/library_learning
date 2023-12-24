mod living_room {
    pub mod entrance {
        pub fn add_to_waitlist() {}

        fn bring_to_table() {}
    }

    pub mod service {
        fn take_order() {}

        pub fn give_order(){}

        fn cash_in() {}
    }

    mod kitchen{
        fn cook_order(){}

        fn correct_wrong_order(){
            cook_order();
            super::service::give_order();
        }
    }
}

pub fn eat_at_restaurant(){
    //crate::living_room::entrance::add_to_waitlist;
    //living_room::entrance::add_to_waitlist;
}