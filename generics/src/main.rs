mod functions;
mod traits;

fn main() {
    functions::show_generic_function();
    functions::show_impl_with_generic();

    traits::show_generic_trait();
    traits::show_generic_using_where();
    traits::show_generic_non_associative_type();
    traits::show_generic_with_associated_types();
}
