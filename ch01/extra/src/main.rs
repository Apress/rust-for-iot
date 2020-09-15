mod variables;
mod ownership;
mod complex_moves;
mod traits;
mod refs;
mod asyn;

// Create the app
fn main() {
    // tag::snippet[]
    println!("Hello, world!");
    // end::snippet[]
    // call_func();
    // variables::run();
    // ownership::run();
    // complex_moves::run();
    // traits::run();
    // refs::run();
    asyn::run();
}

fn call_func() {
    println!("Calling a function")
}
