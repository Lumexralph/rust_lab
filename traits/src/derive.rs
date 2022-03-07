/*
    Traits
    A trait is a collection of methods defined for an unknown type: `Self`
    Traits can be implemented for any data type
*/

struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    // Self::new(name)
    fn new(name: &'static str) -> Self;

    // methods
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Default methods implemented in the trait Animal.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

// methods for the Sheep instance
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    // mutating the instance
    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

// Implement Animal trait for Sheep struct.
impl Animal for Sheep {
    // Self is the implementor type i.e Sheep
    fn new(name: &'static str) -> Self {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baa?"
        } else {
            "baa!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

pub fn show_trait_implementation() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Sheep::new("sheeper");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    // Apparently, we can use the Animal trait too
    let mut sonic: Sheep = Animal::new("super-sonic");

    sonic.talk();
    sonic.shear();
    sonic.talk();
}

// derive
// The compiler is capable of providing basic implementations for
// some traits via the #[derive] attribute. These traits can still
// be manually implemented if a more complex behavior is required.
//
// There are some other derivable traits:
// Comparison traits: Eq, PartialEq, Ord, PartialOrd.
// Clone, to create T from &T via a copy.
// Copy, to give a type 'copy semantics' instead of 'move semantics'.
// Hash, to compute a hash from &T.
// Default, to create an empty instance of a data type.
// Debug, to format a value using the {:?} formatter.


// Returning Traits with dyn
// Rust wants to know the return type of a function
// to know what memory allocation it should make for it,
// so it needs a concrete type. Whenever you want to return
// a trait, we use a Box, which is allocating a pointer on
// the heap memory. We make it a box pointer to a trait.
// A box is just a reference to some memory in the heap.
// Because a reference has a statically-known size, and
// the compiler can guarantee it points to a heap-allocated concrete type,
// we can return a trait from our function.
//
// pointer-to-trait-on-heap write the return type with the dyn keyword, e.g. Box<dyn Animal>

struct Goat {}
struct Cow {}

trait Mammal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Mammal` trait for `Goat`.
impl Mammal for Goat {
    fn noise(&self) -> &'static str {
        "bleeh!"
    }
}

// Implement the `Mammal` trait for `Cow`.
impl Mammal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Mammal, but we don't know which one at compile time.
fn random_mammal(random_number: f64) -> Box<dyn Mammal> {
    if random_number < 0.5 {
        Box::new(Goat {})
    } else {
        Box::new(Cow {})
    }
}

// impl Trait
// can be used in two locations:
// as an argument type
// as a return type
fn parse_csv<R: std::io::BufRead>(reader: R) -> std::io::Result<Vec<Vec<String>>> {
    reader.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                // Split the line separated by commas
                line.split(',')
                    // Remove leading and trailing whitespace
                    .map(|entry| String::from(entry.trim()))
                    .collect() // // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}

// parse csv can also be rewritten as:
// parse_csv_document::<std::io::Empty>(std::io::empty()) will not work with the second example
fn parse_csv_document(reader: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    reader.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                // Split the line separated by commas
                line.split(',')
                    // Remove leading and trailing whitespace
                    .map(|entry| String::from(entry.trim()))
                    .collect() // // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}

// As a return type
// If your function returns a type that implements MyTrait,
// you can write its return type as -> impl MyTrait.

// Supertraits
// Rust doesn't have "inheritance", but you can define a trait as being a superset of another trait
trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

// take in trait that was allocated on the heap
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

pub fn show_return_trait_from_function() {
    let random_number = 0.234;
    let animal = random_mammal(random_number);
    println!("You've randomly chosen a mammal, and it says {}", animal.noise());
}
