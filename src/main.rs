mod greetings;
use greetings::{default_greeting, french, spanish};

fn main() {
    println!("Hello world");
    println!("{}", default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
}
