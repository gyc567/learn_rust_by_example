mod learn_option;
mod turbofish;
use turbofish::*;

use learn_option::*;

fn main() {
    println!("Hello, world!");
    let msg = Some("howdy");

    // Take a reference to the contained string
    if let Some(m) = &msg {
        println!("{}", *m);
    }

    // Remove the contained string, destroying the Option
    let unwrapped_msg = msg.unwrap_or("default message");

    // A list of data to search through.
    let all_the_big_things = [
        Kingdom::Plant(250, "redwood"),
        Kingdom::Plant(230, "noble fir"),
        Kingdom::Plant(229, "sugar pine"),
        Kingdom::Animal(25, "blue whale"),
        Kingdom::Animal(19, "fin whale"),
        Kingdom::Animal(15, "north pacific right whale"),
    ];
    getBigThing(all_the_big_things);
    test_collection();
}
