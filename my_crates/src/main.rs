/*
    In other words, modules do not get compiled individually, only crates get compiled.
    Every content in declared mod in a crate will be exported in place of the mod
    before the compiler, executes it.

    A crate can be compiled into a binary or into a library. By default, rustc will produce
    a binary from a crate.
    This behavior can be overridden by passing the --crate-type flag to lib.
*/

fn main() {
    println!("Hello, world!");
}
