/*
    Specifically, a variable's lifetime begins when it is created and ends
    when it is destroyed. While lifetimes and scopes are often referred to
    together, they are not the same.

    Lifetime is the time for the lender of the borrow, scope is a matter
    of where the borrower is used.

    Explicit annotation
    The borrow checker uses explicit lifetime annotations to determine
    how long references should be valid
    foo<'a> ==> `foo` has a lifetime parameter `'a`

    Similar to closures, using lifetimes requires generics.
    Additionally, this lifetime syntax indicates that the
    lifetime of foo may not exceed that of 'a. Explicit
    annotation of a type has the form &'a T where 'a has already been introduced.

    In cases with multiple lifetimes, the syntax is similar:
    foo<'a, 'b> ==>  `foo` has lifetime parameters `'a` and `'b`
    In this case, the lifetime of foo cannot exceed that of either 'a or 'b.
*/

use std::fmt::Debug;

// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// TODO: remove this function
// A function which takes no arguments, but has a lifetime parameter `'a`.
// fn failed_borrow<'a>() {
//     let _x = 34;
//
//     // ERROR: `_x` does not live long enough
//     let y: &'a i32 = &_x;
//
//     // Attempting to use the lifetime `'a` as an explicit type annotation
//     // inside the function will fail because the lifetime of `&_x` is shorter
//     // than that of `y`. A short lifetime cannot be coerced into a longer one.
// }

pub fn show_lifetime_with_explicit_annotation() {
    let (three, six) = (3, 6);

    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&three, &six);
    // Any input which is borrowed must outlive the borrower.
    // the value we are borrowing from must outlive the borrower of that value.
    // This means that the lifetime of three and six must outlive the function print_refs.

    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
    // TODO: remove this function
    // failed_borrow();
}

// Functions with lifetimes
// any reference must have an annotated lifetime.
// any reference being returned must have the same lifetime as an input or be static.

// One input reference with lifetime `'a` which must live
// at least as long as the function.
fn print_one<'a>(value: &'a i32) {
    println!("`print_one`: x is {}", value);
}

// Mutable references are possible with lifetimes as well.
// a function with side effect
fn add_two<'a>(value: &'a mut i32) {
    *value += 2;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but
// in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// The above is invalid: `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

// Methods are annotated similarly to functions:
#[derive(Debug)]
struct Owner(&'static str, &'static str);

impl Owner {
    fn add_last_name<'a>(&'a mut self, last_name: &'static str) {
        self.1 = last_name;
    }

    fn print_name<'a>(&'a self) {
        println!("name: {:?}", self)
    }
}

pub fn show_functions_with_lifetime() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_two(&mut t);
    print_one(&t);

    // methods
    let mut owner = Owner("Lumex", "");
    owner.add_last_name("Ralph");
    owner.print_name();
}

// Annotation of lifetimes in structures are also similar to functions:

// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrow<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    NumRef(&'a i32),
}

// Traits
#[derive(Debug)]
struct BorrowedV2<'a> {
    x: &'a i32,
}

impl<'a> Default for BorrowedV2<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

// Lifetime with Bounds
// T: 'a: All references in T must outlive lifetime 'a.
// T: Trait + 'a: Type T must implement trait Trait and all references in T must outlive 'a.

// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

// Lifetime Coercion
// A longer lifetime can be coerced into a shorter one so that it works
// inside a scope it normally wouldn't work in. This comes in the form
// of inferred coercion by the Rust compiler, and also in the form of
// declaring a lifetime difference

// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
// taking in 2 references with different lifetimes
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'a i32) -> &'b i32 {
    first
}

pub fn show_lifetime_in_struct() {
    // source for the lenders, they will be borrowed from.
    // the lifetime(initialization and destroy) of these
    // variable would determine if they can be used as references.
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrow { x: &x, y: &y };
    let num_reference = Either::NumRef(&x);
    let number = Either::Num(x);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", num_reference);
    println!("x is *not* borrowed in {:?}", number);

    let b: BorrowedV2 = Default::default();
    println!("b is {:?}", b);

    // lifetime with trait bounds
    let name = "lumex";
    let ref_name = Ref(&name);

    print_ref(&ref_name);
    print(ref_name); // now the value is moved the reference no longer exists

    let first = 2; // longer lifetime

    {
        let second = 3; // shorter lifetime

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}


// Static Lifetime
// Rust has a few reserved lifetime names. One of those is 'static.
// You might encounter it in two situations:
// A reference with 'static lifetime: let s: &'static str = "hello world";

// 'static as part of a trait bound:
fn generic<T>(t: T) where T: 'static {}

// Static Reference Lifetime
// As a reference lifetime 'static indicates that the data pointed to by the
// reference lives for the entire lifetime of the running program.
// It can still be coerced to a shorter lifetime.
//
// There are two ways to make a variable with 'static lifetime, and both are
// stored in the read-only memory of the binary:
// Make a constant with the static declaration: static PI: float64 = 3.172;
// Make a string literal which has type: &'static str

// Make a constant with `'static` lifetime.
static NUM: i32 = 45;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

pub fn show_static_lifetime_reference() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    // since static lifetime stays till running program, NUM is still accessible
    println!("NUM: {} stays accessible", NUM);
}