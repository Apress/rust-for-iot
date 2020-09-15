

mod people { 
    // and everything on the stuct is needed to be public if used outideh temodule
    // tag::person[]
    pub struct Person {
        pub name: &'static str,
        pub address: &'static str
        //pub extensions: TypeMap,
    }

    // pub is implied and not needed here
    impl Person {
        //fn address
        pub fn say_hello(&self) -> String {
            format!("Hello {}", self.name)
        }
    }
    // end::person[]

    // tag::student[]
    pub trait Student {
        // sets the name
        fn new(name: &'static str) -> Self;

        // gets the name
        fn name(&self) -> &'static str;

        // enrolls in the class
        fn enroll(&self, class: &'static str) -> bool;
    }
    // end::student[]

    pub trait Faculty {
        // sets the name
        fn new(name: &'static str) -> Self;

        fn teach(&self, class: &'static str) -> bool {
            println!("Teach the student {}", class);
            true
        }
    }

    // tag::useTrait[]
    impl Student for Person { // <1>
        fn new(name: &'static str) -> Person { 
            Person { name: name, address: "" }
        } // <2>

        // gets the name
        fn name(&self) -> &'static str {
            self.name
        } // <3>

        // enrolls in the class
        fn enroll(&self, class: &'static str) -> bool {
            println!("Enroll the student in {}", class);
            true
        }
    }
    // end::useTrait[]
}

mod run1 {
    use super::people::Person;

    fn add_person() {
        // have to include all the fields
        let p = Person {name: "joseph", address: "123 Main Street Phoenix AZ" };

        println!("Person :: {}", p.name);
    }

    fn add_person_with_trait() {
        // tag::runTrait[]
        use super::people::Student;
        
        // have to include all the fields
        let mut p: Person = Student::new("joseph");

        println!("Person W/Trait:: {}", p.name);
        p.enroll("CS 200");
        // end::runTrait[]
    }

    pub fn run() {
        add_person();
        add_person_with_trait();
    }
}

// tag::runTrait2[]
mod run2 {
    use super::people::Person;
    use super::people::Student; // <1>

    pub fn run(person: Person) {
        person.enroll("CS 200");
    }
}
// end::runTrait2[]

pub fn run() {
    // Type annotation is necessary in this case.
    // let mut dolly: Sheep = Animal::new("Dolly");
    // // TODO ^ Try removing the type annotations.

    // dolly.talk();
    // dolly.shear();
    // dolly.talk();

    //let mut joseph: Undergrad = Student::new("joseph");
    run1::run();
    run2::run(people::Person{name: "joseph", address: "Here;s our address"});
}

fn main() {
    run();
}

/*
struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

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

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}
*/