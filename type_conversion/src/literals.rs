use std::convert::{From, TryFrom, TryInto};
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

pub fn display_literals() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    /*
    Unsuffixed literals, their types depend on how they are used
    */
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("\nsize of `x` in bytes: {}, bits: {}",
        std::mem::size_of_val(&x), std::mem::size_of_val(&x) * 8);
    println!("size of `y` in bytes: {}, bits: {}",
        std::mem::size_of_val(&y), std::mem::size_of_val(&y) * 8);
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}, bits: {}\n",
        std::mem::size_of_val(&f), std::mem::size_of_val(&f) * 8);
}

/*The type statement can be used to give a new name to an existing type. Types must have UpperCamelCase names, or the compiler will raise a warning

This is a way to create a new type from another type, but having an
underlying concrete type.
*/
// `NanoSecond` is a new name for `u64`.
type NanoSecond = u64;
type Inch = u64;

// The main use of aliases is to reduce boilerplate;
pub fn display_aliasing() {
    let nanoseconds: NanoSecond = 5;
    let inches: Inch = 2;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

// If the From trait is implemented for a type, (Type::from())
// it makes the Into trait to be able to implicitly call the
// From trait implementation when type.into() is called.
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn use_from_and_into() {
    // for example
    let lumex_str = "lumex"; // literal string
    // It uses the From trait implementation to create a
    // String type allocated from the heap.
    let lumex_string = String::from(lumex_str);
    println!("\nliteral (from stack): {}, from heap: {}",
                lumex_str, lumex_string);

    let num = Number::from(30);
    println!("my Number is {:?}\n", num);

    // Into trait will implicitly call the From trait.
    let value = 45;
    let number: Number = value.into();
    println!("my Number(Into) is {:?}", number);
}

/* TryFrom and TryInto are generic traits for converting between types.
Unlike From/Into, the TryFrom/TryInto traits are used for fallible
conversions, and as such, return Results.

This is understandable since something can go wrong.
*/
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

pub fn display_try_from_into_conversion() {
    // TryFrom Trait
    assert_eq!(EvenNumber::try_from(10), Ok(EvenNumber(10)));
    assert_eq!(EvenNumber::try_from(7), Err(()));

    // TryInto Trait
    let result: Result<EvenNumber, ()> = 10.try_into();
    assert_eq!(result, Ok(EvenNumber(10)));

    let val = match result {
        Ok(EvenNumber(value)) => value,
        Err(()) => -0,
    };

    println!("even-number: {}", val);

    let result: Result<EvenNumber, ()> = 7.try_into();
    assert_eq!(result, Err(()));
}

// To convert any type to a String is as simple as
// implementing the ToString trait for the type.
// Instead of implementing ToString, we should instead,
// implement the fmt::Display trait which will implicitly
// provides the ToString trait, any type that implements
// the Display trait, also implements the ToString trait.

#[derive(Debug)]
struct Circle {
    radius: u16,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// So now, to be able to convert a string to a type,
// you need to implement the FromStr trait, this is
// called using the parse function.

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_arr: Vec<&str> = s.split(' ').collect();
        let str_val = str_arr[str_arr.len() - 1];
        let result: Result<u16, Self::Err> = str_val.parse();

        match result {
            Ok(val) => Ok(Circle { radius: val}),
            Err(err) => Err(err),
        }
    }
}

pub fn display_string_conversion() {
    let circle = Circle { radius: 65 };

    println!("{}", circle.to_string());
    // same as
    println!("{}", circle);

    let circle_str = circle.to_string();
    let result = circle_str.parse::<Circle>();

    match result {
        Ok(circle) => println!("\nstring circle: `{}` => real circle: {:?}",
                            circle_str, circle),
        Err(err) => println!("\nerror converting to circle {:?}", err),
    }

    let turbo_parsed = "20".parse::<i32>().unwrap();
    let parsed: i32 = "30".parse().unwrap();
    let sum = parsed + turbo_parsed;
    println!("\nSum: {:?}", sum);
}

/*
Blocks are expressions too, so they can be used as values in assignments.
The last expression in the block will be assigned to the place expression
such as a local variable. However, if the last expression of the block
ends with a semicolon, the return value will be ().
*/
pub fn show_expressions() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
