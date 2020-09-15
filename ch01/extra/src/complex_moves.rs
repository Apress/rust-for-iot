
pub fn run() {
    println!("------ Complex Moves ------");

    let p1 = Person {
        name: "Joseph",
        age: 32
    };
    println!("P1 : {:?}", p1);
    take_person(p1);
    let p2 = p1;
    println!("P1 : {:?}", p1);

    // Testing one item that doesnt have Copy/Clone 
    // so it will fail
    let stud = Student {
        name: "Joseph",
        age: 32
    };
    //take_student(stud);
    let stud2 = stud; 

    // Testing one item that doesnt have Copy/Clone 
    // so it will fail    
    take_student_works(&stud2);
    let stud3 = stud2; 
}

fn take_person(person :Person) {
    println!("Took the person {:?}", person);
}

fn take_student(student :Student) {
    println!("Took the student {:?}", student);
}

fn take_student_works(student :&Student) {
    println!("Took the student {:?}", student);
}

// Debug can do automatically, Display apparently  requires more work
// tag::person[]
#[derive(Debug,Copy, Clone)]
struct Person<'a> {
    name: &'a str,
    age: i32,
}
// end::person[]

// tag::student[]
#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    age: i32,
}
// end::student[]