pub enum Kingdom {
    Plant(u32, &'static str),
    Animal(u32, &'static str),
}

pub fn getBigThing(mut all_the_big_things: [Kingdom; 6]) {
    // We're going to search for the name of the biggest animal,
    // but to start with we've just got `None`.
    let mut name_of_biggest_animal = None;
    let mut size_of_biggest_animal = 0;
    for big_thing in &all_the_big_things {
        match *big_thing {
            Kingdom::Animal(size, name) if size > size_of_biggest_animal => {
                // Now we've found the name of some big animal
                size_of_biggest_animal = size;
                name_of_biggest_animal = Some(name);
            }
            Kingdom::Animal(..) | Kingdom::Plant(..) => (),
        }
    }

    match name_of_biggest_animal {
        Some(name) => println!("the biggest animal is {}", name),
        None => println!("there are no animals :("),
    }
}
#[test]
fn test_map_method() {
    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());

    assert_eq!(maybe_some_len, Some(13));
}
#[test]
fn test_map_or_method() {
    let x = Some("foo");
    assert_eq!(x.map_or(42, |v| v.len()), 3);

    let x: Option<&str> = None;
    assert_eq!(x.map_or(42, |v| v.len()), 42);
    fn sq(x: u32) -> Option<u32> {
        Some(x * x)
    }
    fn nope(_: u32) -> Option<u32> {
        None
    }

    assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
    assert_eq!(Some(2).and_then(sq).and_then(nope), None);
    assert_eq!(Some(2).and_then(nope).and_then(sq), None);
    assert_eq!(None.and_then(sq).and_then(sq), None);
}
