/*
 Borrowing
 Most of the time, we'd like to access data without taking ownership over it.
 To accomplish this, Rust uses a borrowing mechanism. Instead of passing objects
 by value (T), objects can be passed by reference (&T).

 The compiler statically guarantees (via its borrow checker) that references always
 point to valid objects. That is, while references to an object exist,
 the object cannot be destroyed.
*/

// This function takes ownership of a box and destroys it
// parameter is passed by value.
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32, parameters passed by reference.
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

pub fn show_borrowing() {
    let boxed_i32 = Box::new(5_i32); // from the heap (pointer)
    let stacked_i32 = 6_i32; // from the stack

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // TODO: remove this line of code, it will error
        // we cannot move and transfer the ownership of this data
        // because another we are still borrowing it later in the code
        // eat_box_i32(boxed_i32);

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.

        // we can successfully move it here
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);
}

// Mutable Reference
// Some situations will arise when we want to borrow a mutable data
// and not transfer its ownership, we just want to update the data
// keep it still in its memory location.
//
// Mutable data can be mutably borrowed using &mut T.
// This is called a mutable reference and gives read/write access to the borrower.
// In contrast, &T borrows the data via an immutable reference, and the borrower
// can read the data but not modify it:

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
// no ownership is transferred, just borrowing not moving.
// I have a read access.
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// This function takes a reference to a mutable book and modifies it.
// I have a read and write access to the Book data.
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

pub fn show_borrowing_with_mutable_reference() {
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // to be able to modify immutable book,
    // copy (it has the Copy trait) the data and make it mutable.
    // if not for the Copy and Clone trait, the book will be moved.
    let mut mutable_book = immutabook;

    // Immutably borrow an immutable object
    borrow_book(&immutabook);

    // Immutably borrow a mutable object
    borrow_book(&mutable_book);

    // Borrow a mutable object as mutable
    new_edition(&mut mutable_book);

    // Error! Cannot borrow an immutable object as mutable
    // new_edition(&mut immutabook);
}

// Aliasing
// Data can be immutably borrowed any number of times, but while immutably borrowed,
// the original data can't be mutably borrowed.
// On the other hand, only one mutable borrow is allowed at a time. The original data
// can be borrowed again only after the mutable reference has been used for the last time.
// i.e the use goes out of scope.

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn show_borrowing_scenarios() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    // make a borrow by referencing the data location.
    // except the data or variable has a Clone or Copy trait,
    // the data will be moved so we do a reference.
    let borrowed_point = &point;
    let another_borrowed_point = &point;

    // Data can be accessed via the references and the original owner
    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrowed_point.y, point.z);

    // TODO: remove the below line
    // we can't borrow point data as a mutable borrow/reference
    // because there's an immutable borrow below or used later.
    // let mutable_borrow = &mut point;

    // The borrowed values are used again here
    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrowed_point.y, point.z);

    // The immutable references are no longer used for the rest of the code so
    // it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;

    // Change data via mutable reference
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // we can't have immutable borrow/reference here because the mutable reference
    // or borrow is still used below after the below code
    // TODO: remove this line below
    // let y = &point;
    // println!("Point Z coordinate is {}", point.z); // an immutable reference here too.

    // Ok! Mutable references can be passed as immutable to `println!`
    println!("Point has coordinates: ({}, {}, {})",
             mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // since the mutable reference is no longer used in the remaining part of the code,
    // we can reborrow point again.
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}