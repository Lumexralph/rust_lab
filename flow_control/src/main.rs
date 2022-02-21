mod if_else;
mod loops;
mod pattern_matching;

fn main() {
    if_else::display_if_else(20);
    if_else::display_if_let();
    if_else::display_while_let();

    loops::display_for_loop();
    loops::display_labelled_loops();
    loops::display_loop();
    loops::display_loop_with_return_value();

    pattern_matching::display_regular_match(16);
    pattern_matching::display_tuple_destructuring_match((0, 20, 10));

    let colour = pattern_matching::Color::Red;
    pattern_matching::display_match_with_enums(colour);
    pattern_matching::display_pointer_ref_match();
    pattern_matching::display_struct_match();
    pattern_matching::display_match_guard((4, 4), 30);
    pattern_matching::display_match_with_binding(40, Some(53));
}
