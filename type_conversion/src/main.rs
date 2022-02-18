mod casting;
mod literals;

fn main() {
    casting::display_type_casting();
    literals::display_literals();
    literals::display_aliasing();
    literals::use_from_and_into();
    literals::display_try_from_into_conversion();
    literals::display_string_conversion();
    literals::show_expressions();
}
