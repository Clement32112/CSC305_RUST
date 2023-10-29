mod greetings;
extern crate new_lib;

use greetings::{default_greeting, french, spanish};
use new_lib::greeting_from_lib;
fn main() {
    println!("Hello world");
    println!("{}", default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", greeting_from_lib());
}
