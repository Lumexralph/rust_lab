mod visibility;
mod super_and_self;

fn main() {
    visibility::display_module_visibility();
    super_and_self::my::indirect_call();
}
