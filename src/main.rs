mod greetings;
extern crate data_type_example;
extern crate new_lib;

use data_type_example::how_to_hold_data;
use how_to_hold_data::primitive::{scalar,compound};
use how_to_hold_data::derived::user_derived;

//use data_type_example::how_to_hold_data::primitive::compound;
use greetings::{english, french, spanish};
use new_lib::greeting_from_lib;

fn main() {
    //Primitive
    
    //Scalar
    scalar::integer::run();
    scalar::unsigned_integer::run();
    scalar::float::run();
    scalar::textual::run();

    //Compound
    compound::tuple::run();
    compound::array::run();
    compound::slice::run();
   
    //Derived
   //user_derived::      
    
    println!("{}", english::default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", greeting_from_lib());

}
