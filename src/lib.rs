mod restaurant;

pub use crate::restaurant::living_room::entrance;
pub use crate::restaurant::living_room::kitchen;

pub fn eat_at_restaurant(){

    let mut meal = kitchen::Breakfast::in_summer("Seigle");
    meal.grilled_toast = String::from("blé");
    println!( "Je voudrais une tartine grillée au {}, s'il vous plaît.", meal.grilled_toast);

    let order1 = kitchen::Appetizer::Salad;
    let order1 = kitchen::Appetizer::Soup;

    entrance::add_to_waitlist();
    entrance::add_to_waitlist();
    entrance::add_to_waitlist();
}

//use the same name for 2 use import

use std::fmt::Result;
use std::io::Result as IoResult;
//or
use std::fmt;
use std::io;


//first method

//fn function_with_first_method_using_fmt() -> Result {

//}

//fn function_with_first__method_using_io() -> IoResult<()> {

//}

//second method

//fn function_using_second_method_using_fmt()-> fmt::Result{

//}

//fn function_using_second_method_using_io()-> io::Result<()>{

//}

//multi import
use std::{cmp::Ordering, alloc};

//sub-name import
use std::env::{self, consts};

//global import
use std::collections::*;