/*
    Ownership
    Because variables are in charge of freeing their own resources,
    resources can only have one owner. This also prevents resources
    from being freed more than once.
    Note that not all variables own resources

    When doing assignments (let x = y) or passing function arguments by value (foo(x)),
    the ownership of the resources is transferred.
    In Rust-speak, this is known as a move.

    After moving resources, the previous owner can no longer be used.
    This avoids creating dangling pointers.
*/

// This function takes ownership of the heap allocated memory `c`
// because the parameter was passed by value.
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

pub fn show_ownership() {
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    // both x and y are still accessible
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it. This is a move!
    // `a` can no longer access the data, because it no longer owns the
    // heap memory, any attempt to use `a` will cause an Error!
    let b = a;

    // TODO: this will error
    // println!("a contains: {}", a);

    // calling this function, takes parameter by value,
    // so calling the function causes a move because
    // ownership of `b` heap memory has be transferred
    // to the function, any attempt to use `b` after the
    // function is called will lead to an Error
    // you can't borrow a moved data.
    destroy_box(b);

    // TODO: this will error
    // println!("b contains: {}", b);
}

// Mutability of data can be changed when ownership is transferred.

pub fn show_mutability() {
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);
}

// Partial Moves
// Within the destructuring of a single variable, both by-move and by-reference
// pattern bindings can be used at the same time. Doing this will result in a
// partial move of the variable, which means that parts of the variable will
// be moved while other parts stay. In such a case, the parent variable cannot
// be used afterwards as a whole, however the parts that are only referenced
// (and not moved) can still be used.

#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}

pub fn show_partial_moves() {
    let person = Person {
        // all allocations on the heap.
        name: String::from("Lumex"),
        age: Box::new(23),
    };

    // destructuring name by value(move), age by reference borrow
    // person will have age available but name can't be used because
    // it has been moved(memory location and owner) to the new variable.
    let Person { name, ref age } = person;

    // we store the age variable on the heap to illustrate the partial move:
    // deleting ref in the above code would give an error as the ownership
    // of person.age would be moved to the variable age.
    // If Person.age were stored on the stack, ref would not be required as
    // the definition of age would copy the data from person.age without moving it.

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // we can't access the whole struct in person variable anymore.
    // Error! borrow of partially moved value: `person` partial move occurs
    // println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}
