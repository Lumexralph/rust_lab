// Closures are functions that can capture the enclosing environment. For example,
// a closure that captures the x variable: |val| => val + x

/*
Capturing

This allows capturing to flexibly adapt to the use case, sometimes moving
and sometimes borrowing. Closures can capture variables:
by reference: &T
by mutable reference: &mut T
by value: T

They preferentially capture variables by reference and only go lower when required.
*/
use std::mem;

pub fn display_closure_capturing() {
    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow by reference.
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`, and we are also taking an immutable
    // reference through the _reborrow variable.
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    // which means the closure has been called and it is out of scope,
    // popped off the stack, so it has no reference to any variable.
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error because it holds a mutable reference.
    // this will fail because we have another closure holding mutable reference to count.
    // Below will work if the closure is dealing with an immutable reference.
    // let _reborrow_count = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    // A non-copy type.
    let movable = Box::new(5);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume(); // can only be called once, if called twice, it'll panic.
    // this should fail.
    // consume();

    let haystack = vec![3, 4, 6];

    // Using move before vertical pipes forces closure to take ownership of
    // captured variables, in this case `haystack`:
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // Uncommenting below line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.
    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available

    // TODO:  remove `move` from the closure and uncomment the below line.
    // println!("There're {} elements in vec", haystack.len());
}

/*
Closures as input parameters.

When taking a closure as an input parameter, the closure's complete type must be annotated
using one of a few traits. In order of decreasing restriction, they are:
Fn: the closure captures by reference (&T)
FnMut: the closure captures by mutable reference (&mut T)
FnOnce: the closure captures by value (T)
*/

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {

    // call the closure
    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_number<F>(f: F, number: i32) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {
    f(number)
}

// A function which takes a closure using mutable reference
// it returns a String.
fn change_to_upper<F: FnOnce(&str) -> String>(f: F, word: &str) -> String {
    let mut output = f(word).to_uppercase();
    output.push_str("...");

    return output;
}

fn apply_append_to_array<F>(mut f: F, arr: Vec<&str>) where
    F: FnMut(Vec<&str>){
    f(arr);
}

pub fn display_closure_as_input_parameters() {
    // A non-copy type.
    let greeting = "hello";
    // `to_owned` creates owned data from borrowed one
    // `farewell` get is own data from the borrowed data in `greeting`.
    let mut farewell = greeting.to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        // we borrowed it.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    let triple = |val| 3 * val;

    println!("number: {}, tripled -> {}", 4, apply_to_number(triple, 4));

    let add_world = |word: &str| {
        let mut added_word = String::from(word);
        added_word.push_str(" World");
        return added_word;
    };

    let append_to_arr = |mut arr: Vec<&str>| {
        arr.push("ball");
        println!("changed array: {:?}", arr);
    };
    println!("transform word: {}, to -> {}", "hello", change_to_upper(add_world, "hello"));

    let mut arr = vec!["foot"];
    apply_append_to_array(append_to_arr, arr);
}

/*
    Closures as output parameters.

     Closures as input parameters are possible, so returning closures as output parameters
     should also be possible. However, anonymous closure types are, by definition, unknown,
     so we have to use impl Trait to return them.

     the move keyword must be used, which signals that all captures occur by value (copied).
     This is required because any captures by reference would be dropped as soon as
     the function exited, leaving invalid references (no valid data) in the closure.
*/

fn create_fn() -> impl Fn() {
    // we need to create our own copy.
    // if we do this let text = "string";
    // the text will be dropped before the function
    // returns, meaning the captured text value in
    // the closure will not exists, invalid reference.
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

pub fn display_closure_as_output() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}