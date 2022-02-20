pub fn display_if_else(n: i32) {
    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
        }; // All let bindings need the semicolon.

    println!("{} -> {}", n, big_n);
}
