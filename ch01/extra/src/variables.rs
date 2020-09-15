#![allow(dead_code)]

// extern crate t_bang;
// use t_bang::*;


fn main() {
    run();
}

pub fn run() {
    println!("------ Variables ------");
    variable_set();
    mutable();
    more_variables();
    tuples();
    more_tuples(); 
}
pub fn change_variable() {
    // tag:mut2[]
    let mut x :i32 = 4;
    x = 5;
    // end:mut2[]
    
}
fn tuples() {
    println!("-Tuples-");
    // tag:tuples[]
    let x = (1,2,3);
    let (a, b) = (35, "entry");
    println!("X : {:?}", x);
    println!("X : {:?} / {:?}", a, b);
    // end:tuples[]
}

// tag:more_tuples[]
fn more_tuples() {
    let (a,b) = response_with_tuples();
    println!("a / b :: {:?} / {:?}", a, b);
}

fn response_with_tuples() -> (String, u32) {
    (String::from("ok"), 32)
}
// end:more_tuples[]

fn variable_set() {
    #![allow(unused_variables)]
    // tag::vars[]
    let a = 3;   
    let b = 'J';
    let c = "Joseph";     
    // end::vars[]

    // tag::types[]
    let v :u32 = 3;    
    let x :char = 'J';
    let y :&str = "Joseph";
    let z :i64;
    // end::types[]
    
    println!("Our type : {} / {} / {}", b, a, x); 
    println!("x : {} / y : {}", x, y);
}

fn more_variables() {
    // tag::other_types[]
    let x :bool = true;
    let y: char = '\u{00e9}';
    let z: char = 'a';
    let a: char = '😻';

    println!("Extra Chars: {} , {}, {}, {} ", x, y, z, a);
    // end::other_types[]
}


fn mutable() {
    // tag::mut[]
    let mut x = 3; // <1>
    x = 5;

    let mut y :i32; // <2>
    y = 25;

    let mut z :i32 = 3; // <3>
    z = 2; 
    println!("X : {} / Y: {} / Z: {}",x ,y, z);
    // end::mut[]
}
