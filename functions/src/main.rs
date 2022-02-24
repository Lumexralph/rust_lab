mod functions;
mod closures;


fn main() {
    functions::display_functions_and_methods();
    functions::display_high_order_function();

    closures::display_closure_capturing();
    closures::display_closure_as_input_parameters();
    closures::display_closure_as_output();
}
