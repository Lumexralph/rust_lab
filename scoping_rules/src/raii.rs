/*
    RAII (Resource Acquisition Is Initialization)
    Rust enforces RAII (Resource Acquisition Is Initialization),
    so whenever an object goes out of scope, its destructor is
    called and its owned resources are freed.
*/

fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(45_i32);

    // `_box1` is destroyed here, and memory gets freed
    // because the it goes out of scope as the function
    // returns or finishes execution.
}

// we can double check for memory errors using valgrind: https://valgrind.org/info/
// rustc raii.rs && valgrind ./raii

// Destructor
// The notion of a destructor in Rust is provided through the Drop trait.
// The destructor is called when the resource goes out of scope.
// This trait is not required to be implemented for every type,
// only implement it for your type if you require its own destructor logic
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

pub fn show_raii() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
        // as it goes out of scope.
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed

    let x = ToDrop;
    println!("Made a ToDrop!");
}
