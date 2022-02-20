#![allow(unreachable_code)]

// a loop keyword to indicate an infinite loop
pub fn display_loop() {
    let mut count = 0_u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration,
            // move to the next count.
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}

// loops must be annotated with some 'label,
// and the label must be passed to the break/continue statement.

pub fn display_labelled_loops() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

pub fn display_loop_with_return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

// The for in construct can be used to iterate through an Iterator.
// One of the easiest ways to create an iterator is to use 
// the range notation a..b.
pub fn display_for_loop() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // by default the for loop will apply the into_iter
    // function to the collection. This means that the
    // collection must implement the Iterator trait
    // to be used with a for in loop.
    // into_iter, iter and iter_mut all handle the
    // conversion of a collection into an iterator in different ways

    // iter
    // This borrows each element of the collection through each iteration.
    // Thus leaving the collection untouched and available
    // for reuse after the loop.
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter
    // This consumes the collection so that on each iteration 
    // the exact data is provided. Once the collection has been
    // consumed it is no longer available for reuse as it has 
    // been 'moved' within the loop.
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names); // it will fail

    // iter_mut
    // This mutably borrows each element of the collection,
    // allowing for the collection to be modified in place.
    let mut my_names = vec!["Bob", "Frank", "Ferris"];

    for name in my_names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "A Rustacean",
            _ => "Holla",
        }
    }

    println!("names: {:?}", my_names);
}
