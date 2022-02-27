mod ownership;
mod raii;
mod borrowing;
mod ref_pattern;

fn main() {
    raii::show_raii();

    ownership::show_ownership();
    ownership::show_mutability();
    ownership::show_partial_moves();

    borrowing::show_borrowing();
    borrowing::show_borrowing_with_mutable_reference();
    borrowing::show_borrowing_scenarios();

    ref_pattern::show_ref_pattern();
}
