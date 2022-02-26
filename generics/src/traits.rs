use std::fmt::{ Display, Debug};

// Non-copyable types.
struct Empty;
struct Null;

// A trait generic over `T`.
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    fn double_drop(self, _: T)  where Self: Sized {}
}

// implement this trait for any type `T` and caller `U` of this trait's method
// caller meaning any instance/type that calls the trait method.
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments,
    // deallocating both when it is out of scope.
    fn double_drop(self, _: T) {}
}

// Bounding a type T to implement a particular Trait
// When working with generics, the type parameters often must use
// traits as bounds to stipulate what functionality a type implements.

fn print<T: Display>(data: T) {
    println!("print {}", data);
}

// Bounding restricts the generic to types that conform to the bounds.
struct S<T: Display>(T);

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T)   -> &str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

pub fn show_generic_trait() {
    let empty = Empty;
    let null = Null;

    // Deallocate `empty` and `null`.
    // they will get dropped and inaccessible after this
    // due to the ownership by the double_drop method of
    // the caller U - `empty` and the passed in type T - `null`.
    empty.double_drop(null);

    // TODO: no longer accessible, remove
    // empty;
    // null;

    print("hello world");

    // Error! `Vec<T>` does not implement `Display`. This
    // specialization will fail.
    // let s = S(vec![1]);

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec);
}

// using multiple bounds
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Display: {0}, Debug: {:?}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}, u: {:?}", t, u);
}

// this could be better display
// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// as
// impl <A, D> MyTrait<A, D> for YourType where
//     A: TraitB + TraitC,
//     D: TraitE + TraitF {}

trait PrintInOption {
    fn print_in_option(self);
}

// A way for adding a behaviour to all the types in your code.
// Because we would otherwise have to express this as `T: Debug` or
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

pub fn show_generic_using_where() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}

// Associative Types helps improve the readability of your code
// and solve the following problem when using generics.

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
trait ContainsA<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    // Explicitly requires `A` and `B`.
    fn first(&self) -> i32;
    // Doesn't explicitly require `A` or `B`.
    fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
}

struct ContainerA(i32, i32);

impl ContainsA<i32, i32> for ContainerA {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
       self.1
    }
}

// `C` contains `A` and `B`. In light of that, having to express `A` and
// `B` again is a nuisance.
fn difference<A, B, C>(container: &C) -> i32 where
    C: ContainsA<A, B> {
    container.last() - container.first()
}

pub fn show_generic_non_associative_type() {
    let number_1 = 3;
    let number_2 = 10;

    let container = ContainerA(number_1, number_2);

    println!("Does container contain {} and {}: {}",
             &number_1, &number_2,
             container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

// The Problem of A trait that is generic over its container type
// has type specification requirements - users of the trait must
// specify all of its generic types.
// We can reimplement the ContainsA with associated types.

// The use of "Associated types" improves the overall readability of code
// by moving inner types locally into a trait as output types.

// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for
// aliases).
trait ContainsB {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

struct ContainerB(i32, i32);

impl ContainsB for ContainerB {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// Note that functions that use the trait Contains are no longer required
// to express A or B at all:
fn difference_b<C: ContainsB>(container: &C) -> i32 {
    container.last() - container.first()
}

pub fn show_generic_with_associated_types() {
    let number_1 = 3;
    let number_2 = 10;

    let container = ContainerB(number_1, number_2);

    println!("Does container contain {} and {}: {}",
             &number_1, &number_2,
             container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference_b(&container));
}