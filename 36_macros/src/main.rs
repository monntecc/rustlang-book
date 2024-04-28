use macros::cr_vec;

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    /* Declarative macros */
    let v1: Vec<u32> = vec![1, 2, 3];
    let v2: Vec<&str> = vec!["a", "b", "c"];

    // Create a vector using own macro
    let v3: Vec<i32> = cr_vec![-1, -5, 5];
    println!("{:#?}", v3); // [-1, -5, 5], WORKS!
}
