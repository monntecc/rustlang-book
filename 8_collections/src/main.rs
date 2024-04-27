#[allow(dead_code)]
#[allow(unused_variables)]
use std::collections::HashMap;
use std::hash::Hash;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Normal array initialization
    let a = [1, 2, 3];

    /* VECTORS */

    // First is the Vector, it is dynnamical growth size array,
    // that can hold every type inside.
    // Vectors are stored on the heap, and will be dropped when they go out of scope
    let mut v: Vec<i32> = Vec::new();
    // To modify a vector, we need to make it `mutable`
    v.push(1);
    v.push(2);
    v.push(3);

    // If we want to create a vector with some values at the start,
    // Rust have convinient macro for that
    {
        // Allocate memory on the heap for the vector
        let v2 = vec![1, 2, 3];
        // Drop memory from the heap, clear it (out of scope)
    }

    // Accesing elements inside the vector.
    // First is directly accessing by indexing, e.g:
    let second = &v[1]; // we accessing second element in the vector (starting from 0 index)
    println!("Second value of vector is = {}", second);

    // Also we can use `.get()` function from vector,
    // which is return an option, e.g:
    let first = v
        .get(0)
        .expect("Vector does not contains value in provided index!");
    println!("First value of vector is = {}", first);

    /*
       First accessing aproach is a bad one, because we have not know about size of vector on compile-time.
       For example if we are trying to access not valid element on the array, we would get compile error,
       but the Vector's one - runtime.
    */

    // Iterating over elements in the vector, e.g:
    let mut v = vec![1, 2, 3, 4, 5];
    // To do this, we can use for loop, e.g:
    for i in &v {
        println!("{}", i);
    }
    // Or using v.iter() method
    for i in v.iter() {
        println!("{}", i);
    }
    // Also we could have mutable reference:
    for i in &mut v {
        *i += 50; // We access to value using derefence and modify the value
        println!("New value of i is = {}", i);
    }

    // We also can store enum variats inside the vector,
    // it will help us, if we need to store more than one type at once, e.g:
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Match row, and check if it is on type integer
    match &row[1] {
        SpreadsheetCell::Int(i) => println!("Row value is = {}", i),
        // Otherwise
        _ => println!("Not a integer!"),
    }

    /* STRING */
    // Strings are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new(); // We could define empty string
    let s2 = "initial contents"; // String slices
    let s3 = s2.to_string(); // We could transfer slices to the string
    let s4 = String::from("initial contens"); // Or initialize a new one string with some value

    // Appending to the string:
    // To do this, we need to mark string as mutable, e.g:
    let mut s = String::from("foo");
    s.push_str("bar"); // Push string to the string
    s.push('!'); // Push character to the string
                 // After manipulations, string will be: foobar!

    // We also can use `+` to concat strings
    let s1 = String::from("Hello");
    let s2 = String::from(", World!");
    let s3 = s1 + &s2; // We are moving reference from s1 to s3, and adding by passing reference to the s2

    println!("s3 value is = {}", s3); // Hello, World!

    // Also we can concat string using format macro, e.g:
    let s1 = String::from("Hello");
    let s2 = String::from(", World!");
    let s3 = format!("{}{}", s1, s2);

    // In this example, format macro does not take ownership of s1, s2,
    // so we can use it after s3 varible declaration.

    println!("s3 value is = {}", s3); // Hello, World!

    // Indexing of string (get access to character by index)
    let hello: String = String::from("Привет");
    // let c: char = hello[0]; -- This will call an error, because rust has no idea what types of string we need to get,
    // bytes, scalar values or grapheme clusters, e.g:
    /*
       Bytes
       [224, 164, 168, 224, 164, 174, 224, 164, 165, 141, 224, 164, 164]

       Scalar values
       ['1', 'ы', 'г', 'й', '.']

       Grapheme clusters
       ["П", "р", "и", "в", "е", "т"]
    */
    // Iterate as bytes
    for b in hello.bytes() {
        println!("String value as byte = {}", b);
    }
    // Iterate as characters
    for c in hello.chars() {
        println!("String value as char = {}", c);
    }
    // For grapheme clusers, standart library does not include functionality,
    // so we need to use external library like: `unicode-segmentation`
    for g in hello.graphemes(true) {
        println!("String value as grapheme = {}", g);
    }

    /* HASHMAPS */
    // Hashmaps allow us to store key-value pairs, which is key and values could be of any type.
    // Keys must be unique, but values not.
    // To use it, we need to import from std::collections::HashMap;
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    // Create an hashmap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Insert values to it (must be mutable to modify it)
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // To get values from a hashmap, we need to specify an key:
    let team_name = String::from("blue");
    let score = scores.get(&team_name); // We get an option, with value 10

    // We also can iterate through items in a hashmap using for loop:
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20); // This will overwrite the previous value in key `Blue`

    scores.entry(String::from("Yellow")).or_insert(30); // First, we cheking if `Yellow` key is exists, if not - add the new one with value of 30
    scores.entry(String::from("Yellow")).or_insert(40); // Do the same, but just overwrite value 30 to 40

    // Example of update the value in a hash map based on old value
    let text = "Hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // world key will have value 2, because we have 2 words world in a string
    println!("{:?}", map);
}
