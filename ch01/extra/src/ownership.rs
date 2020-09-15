#![allow(dead_code)]

pub fn run() {
    println!("------ Ownership ------");
    create();
    create_str();
    create_str_and_move();
    not_mine();
    run_outofscope();
}

#[derive(Debug, Copy, Clone)]
struct Number {
    num: i32
}

fn not_mine() {
    let zed = "4";
    let me = zed;
    println!("Zed : {}", zed);

    let n = Number { num : 25i32};
    let mv = n;
    println!("Number : {:?}", n);
}


// Creating and moving ownership with a Primitive
// Nothing will be moved cause it implements a copy.
// tag::create[]
fn create() {
    let x :u32 = 3;
    copy_var_to_method(x);

    println!("X :: {}", x);
}

fn copy_var_to_method(x :u32) {
    println!("x: {}", x);
}
// end::create[]

// pass to another method to take ownership
// tag::create_str[]
fn create_str() {
    let x :String = String::from("Joseph");
    take_ownership_str(x);

    //println!("This would fail : {} ", x);
}

fn take_ownership_str(y :String) {
    println!("x: {}", y);
}
// end::create_str[]

// pass to another method to take ownership
// then return it back
// tag::create_str_and_move[]
fn create_str_and_move() {
    let mut x :String = String::from("Joseph");
    x = take_ownership_str_and_return(x);

    println!("End of method : {} ", x);
}

fn take_ownership_str_and_return(y :String) -> String {
    println!("x: {}", y);
    y
}
// end::create_str_and_move[]

// tag::drop[]
#[derive(Debug)]
struct Person {
    name: String
}

fn run_outofscope() {
    let p = Person { name: "Joseph".to_string()};   // <2>
    move_memory(p);
    println!("Finished");
}

fn move_memory(p: Person) {
    println!("Person: {:?}", p);        // <3>
}


impl Drop for Person {              // <1>
    fn drop(&mut self) {
        println!("Dropping!");      // <4>
    }
}
 // end::drop[]

 // https://www.reddit.com/r/rust/comments/b742vu/if_rust_has_no_garbage_collector_how_does_it/