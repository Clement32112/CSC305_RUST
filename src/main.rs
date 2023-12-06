#[allow(dead_code,unused_imports)]

mod greetings;
extern crate data_type_example;
extern crate new_lib;
extern crate function;

use function::run;

use data_type_example::how_to_hold_data;
use how_to_hold_data::primitive::{scalar,compound};
use how_to_hold_data::derived::user_derived;
use how_to_hold_data::derived::pius_user_derived;
use how_to_hold_data::derived::user_derived::structs::Movable;
use greetings::{english, french, spanish};
use new_lib::greeting_from_lib;


fn view_data_types(){
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
 



}
fn greetins(){
    println!("{}", english::default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", greeting_from_lib());

}

fn point_example()
{    println!("{}", "Hello world");   
    let mut my_point: user_derived::structs::Point = user_derived::structs::Point{x:1,y:2};
    my_point.display_coords();
    println!("");
    my_point.move_left(12);
    my_point.display_coords();

}

fn main() 
{
   function::pointer::run();
    function::run();
    //pius_user_derived::run3(); 
   // pius_user_derived::run();
   // piususer_derived::run2();


}
