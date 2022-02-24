// Functions that "don't" return a value, actually return the unit type `()`
// When a function returns `()`, the return type can be omitted from the
// signature.
// Some functions are connected to a particular type. These come in two forms:
// associated functions, and methods. Associated functions are functions that
// are defined on a type generally, while methods are associated functions
// that are called on a particular instance of a type.

struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0, }
    }

    // new is another associated function tied to a type and not an instance of Point.
    fn new(x: f64, y: f64) -> Point {
        Point { x, y, }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(self: &Self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2) + (y1 - y2)).abs()
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`.
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`, consumes self.
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}

pub fn display_functions_and_methods() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    //rectangle.translate(1.0, 0.0);

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(3), Box::new(6));
    pair.destroy();
}

// HOF - High Order Functions
// HOFs and lazy iterators give Rust its functional flavor
fn is_odd(number: u32) -> bool {
    number % 2 == 1
}

pub fn display_high_order_function() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for number in 0.. {
        let number_squared = number * number;

        if number_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(number_squared) {
            // Accumulate value, if it's odd
            acc += number_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Below is the functional approach of the above accumulation operation.
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|number| number * number)    // square number
            .take_while(|&squared_number| squared_number < upper) // Below upper limit
            .filter(|&squared_number| is_odd(squared_number)) // get only odd numbers
            .fold(0, |acc, squared_number| acc + squared_number); // sum them all

    println!("functional style: {}", sum_of_squared_odd_numbers);
}

// Diverging Functions
// Diverging functions never return. They are marked using !, which is an empty type.
// function, which will never return the control back to the caller.
// It is also the return type of functions that loop forever (e.g. loop {}) like network servers
// or functions that terminate the process (e.g. exit()).
fn foo() -> ! {
    panic!("This call never returns.");
}
