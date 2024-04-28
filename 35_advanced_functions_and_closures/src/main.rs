#[allow(dead_code)]
#[allow(unused_variables)]

/* Function pointers */
fn add_one(x: i32) -> i32 {
    x + 1
}

// We can specify a function as a function parameter,
// using fn keyword, e.g:
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
// Or we can use generic type, e.g:
fn do_twice2<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

/* Returning closures from a functions */
fn returns_closure(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b // will work, if we returns only one type using impl Fn, by if more then 1, cannot
}
// To return different closures, we could use this syntax, e.g:
fn returns_closure2(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }
}

fn main() {
    // Check function pointers
    let answer = do_twice(add_one, 5);
    println!("The answer is from normal: {}", answer);
    // Do the same for generic function
    let answer = do_twice2(add_one, 5);
    println!("The answer is from generic: {}", answer);

    // Another example, e.g::
    let list_of_numbers = vec![1, 2, 3];
    // Except of doing in map `|i| i.to_string()`, we could use function from ToString trait,
    // to_string, e.g:
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("List of strings: {:?}", list_of_strings);

    // Another useful pattern for enums:
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("List of statuses: {:#?}", list_of_statuses); // from 1 to 19
}
