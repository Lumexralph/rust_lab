/*
    Generic
    Generic is a topic that helps us with generalization of parameter types
    and functionalities, can help reduce code duplication too by defining
    a single function for many types.
*/

struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

// Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
// It has been explicitly given the type parameter `A`, but because `A` has not 
// been specified as a generic type parameter for `gen_spec_t`, it is not generic.
fn gen_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
// Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
fn generic<T>(_s: SGen<T>) {}

pub fn show_generic_function() {
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(40));

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type of Tuple(&str, int)
    generic(SGen(("ball", 40)));
}

// Similar to functions, implementations require care to remain generic.
struct G; // Concrete type `G`
struct GenericVal<T>(T); // Generic type `GenericVal`

// `<T>` Must precede the type to remain generic
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

struct GenVal<T> {
    gen_val: T,
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

pub fn show_impl_with_generic() {
    let x = Val { val: 20.0 };
    let y = GenVal { gen_val: "ball" };
    let z = GenVal { gen_val: true };

    println!("{} {} {}", x.value(), y.value(), z.value());
}