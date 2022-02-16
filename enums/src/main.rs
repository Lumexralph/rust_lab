// An attribute to hide warnings for unused code.
#![allow(dead_code)]

mod linked_list;


// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

// type aliases for too long enum names.
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// The most common place you'll see this is in impl blocks using the Self alias.
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// Explicitly `use` each name so they are available without
// manual scoping like typing use std::fmt; at the top
// It can be used for inline import of a crate.
fn display_using_use() {
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    // import everything in Work.
    use crate::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("Rich guys!"),
        Poor => println!("we go make am"),
    }

    match work {
        Civilian => println!("civilian work"),
        Soldier => println!("soldier work"),
    }
}

// enum with implicit discriminator (starts at 0), like iota in Go
enum DaysOfTheWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn display_days_of_the_week() {
    // `enums` can be cast as integers.
    println!("\ndays of the week:\n");
    println!("sunday is {}", DaysOfTheWeek::Sunday as u32);
    println!("monday is {}", DaysOfTheWeek::Monday as u32);
    println!("tuesday is {}", DaysOfTheWeek::Tuesday as u32);
    println!("wednesday is {}", DaysOfTheWeek::Wednesday as u32);
    println!("thursday is {}", DaysOfTheWeek::Thursday as u32);
    println!("friday is {}", DaysOfTheWeek::Friday as u32);
    println!("saturday is {}", DaysOfTheWeek::Saturday as u32);
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// enum with explicit discriminator
fn display_colour() {
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let add = Operations::Add;
    let subtract = Operations::Subtract;

    println!("add - {}", add.run(64, 30));
    println!("substract - {}", subtract.run(64, 30));

    display_using_use();
    display_days_of_the_week();
    display_colour();

    // linked-list
    // Create an empty linked list
    let mut list = linked_list::List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("\nlinked list section -");
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
