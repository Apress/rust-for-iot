pub fn run() {
    let mut x = 5i32;
    println!("1 > {:?}", x);
    alter(&mut x);
    println!("2 > {:?}", x);
}

fn alter(x : &mut i32) {
    *x = 3i32;
}