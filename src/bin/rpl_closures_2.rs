fn main() {
    // These are all valid definitions that will produce the same behavior when they’re
    // called
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    // The add_one_v3 and add_one_v4 lines require the closures to be evaluated
    // to be able to compile because the types will be inferred from their usage.

    // i.e. if the next two lines are commented out then definitions of closures won't compile
    add_one_v3(1);
    add_one_v4(1);

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // type mismatch

    // Closures can capture values from their environment in three ways, which directly
    // map to the three ways a function can take a parameter:
    //   - borrowing immutably
    //   - borrowing mutably
    //   - taking ownership
    // The closure will decide which of these to use based on what the body of the
    // function does with the captured values.

    // Following closure captures an immutable reference to the vector named list because
    // it only needs an immutable reference to print the value

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}