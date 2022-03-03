mod ownership;
mod raii;
mod borrowing;
mod ref_pattern;
mod lifetime;

fn main() {
    // raii::show_raii();
    //
    // ownership::show_ownership();
    // ownership::show_mutability();
    // ownership::show_partial_moves();
    //
    // borrowing::show_borrowing();
    // borrowing::show_borrowing_with_mutable_reference();
    // borrowing::show_borrowing_scenarios();
    //
    // ref_pattern::show_ref_pattern();

    // lifetime::show_lifetime_with_explicit_annotation();
    // lifetime::show_functions_with_lifetime();
    // lifetime::show_lifetime_in_struct();
    lifetime::show_static_lifetime_reference();
}
