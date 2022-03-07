mod derive;
mod iterators;

fn main() {
    derive::show_trait_implementation();
    derive::show_return_trait_from_function();

    iterators::show_iterator_trait();
}
