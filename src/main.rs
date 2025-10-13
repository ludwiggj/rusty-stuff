fn basic_string_moves() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    let y = &mut x;
    println!("The value of y is: {}", y); // y goes out of scope
    println!("The value of x is still: {}", x);

    let s1 = "hello";
    println!("The value of s1 is: {}", s1);

    let s2 = s1;
    println!("The value of s2 is: {}", s2); // s2 goes out of scope
    println!("The value of s1 is still: {}", s1);

    let s3 = String::from("sailor");
    println!("The value of s3 is: {}", s3);

    let s4 = s3;
    println!("The value of s4 is: {}", s4);

    // Compilation error - Value used after being moved [E0382], moved at `let s4 = s3`
    // println!("The value of s3 is still: {}", s3);

    // This doesn't happen when value is 5 or immutable string!

    let s5 = String::from("tailor");
    println!("The value of s5 is: {}", s5);

    let s6 = s5.clone(); // this is how to solve that issue

    println!("The value of s6 is: {}", s6);
    println!("The value of s5 is still: {}", s5);
}

fn ownership_and_functions() {
    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("{some_string}");
        // some_string goes out of scope and `drop` is called. The backing memory is freed.
    }

    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope
        println!("{some_integer}");
        // some_integer goes out of scope. Not our concern!
    }

    // s comes into scope
    let s = String::from("hello");

    // s's value moves into the function...
    takes_ownership(s);
    // ... and so is no longer valid here

    // i.e. the following line doesn't compile - Value used after being moved [E0382]
    // println!("The value of s is: {}", s);

    // x comes into scope
    let x = 5;

    makes_copy(x);
    // Because i32 implements the Copy trait, x does NOT move into the function,
    // so it's okay to use x afterward.

    println!("The value of x is: {}", x);

    // Here, x goes out of scope, then s. Handling x is the compiler's concern.
    // However, because s's value was moved, nothing special happens.
}

fn return_values_and_scope() {
    // gives_ownership moves its return value into the function that calls it
    fn gives_ownership() -> String {
        // some_string comes into scope
        let some_string = String::from("yours");

        some_string
        // some_string is returned and moves out to the calling function
    }

    // This function takes a String and returns a String.
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope
        a_string
        // a_string is returned and moves out to the calling function
    }

    // gives_ownership moves its return value into s1
    let s1 = gives_ownership();

    println!("The value of s1 is: {}", s1);

    // s2 comes into scope
    let s2 = String::from("hello");

    println!("The value of s2 is: {}", s2);

    // s2 is moved into takes_and_gives_back, which also
    // moves its return value into s3
    let s3 = takes_and_gives_back(s2);

    println!("The value of s3 is: {}", s3);

    // Value used after being moved [E0382]
    // println!("The value of s2 is: {}", s2);

    println!("The value of s1 is: {}", s1);

    // s3 goes out of scope and is dropped.
    // s2 was moved, so nothing happens.
    // s1 goes out of scope and is dropped.
}

fn returning_ownership_of_parameters() {
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn string_length_with_borrow() {
    // s is a reference to a String
    // When functions have references as parameters instead of the actual values,
    // we won’t need to return the values in order to give back ownership, because
    // we never had ownership.
    fn calculate_length(s: &String) -> usize {
        // len() returns the length of a String
        s.len() // last line must not be terminated by a semi-colon
        // s goes out of scope. But because s does not have ownership of what
        // it refers to, the String is not dropped.
    }

    let s1 = String::from("sailor");

    let len = calculate_length(&s1);

    println!("The length of '{}' is: {}", s1, len);
}

fn immutable_local() {
    let v = 10;
    println!("The value of v is: {}", v);

    // Cannot assign a new value to an immutable variable more than once [E0384]
    // v = 20;

    let s = String::from("hello");
    println!("The value of s is: {}", s);

    // Cannot assign a new value to an immutable variable more than once [E0384]
    // s = "sailor";
}

fn mutable_local() {
    let mut v = 10;
    println!("The value of v is: {}", v);

    v = 20;
    println!("The value of v is: {}", v);

    let mut s = String::from("hello");
    println!("The value of s is: {}", s);

    // Type mismatch [E0308]
    // Expected: String
    // Found: &str
    // s = "sailor";

    s = String::from("sailor");
    println!("The value of s is: {}", s);
}

fn multiple_immutable_borrows_of_mutable_variable_is_ok() {
    let mut s = String::from("hello"); // mutable,  but compiler indicates that it can be immutable
    let immutable_borrow_1 = &s;
    let immutable_borrow_2 = &s;

    println!("The values are {immutable_borrow_1} and {immutable_borrow_2}");
}

fn multiple_immutable_borrows_of_immutable_variable_is_ok() {
    let s = String::from("hello");
    let immutable_borrow_1 = &s;
    let immutable_borrow_2 = &s;

    println!("The values are {immutable_borrow_1} and {immutable_borrow_2}");
}

fn cannot_borrow_immutable_local_variable_as_mutable_1() {
    let s = String::from("hello");

    // Cannot borrow immutable local variable `s` as mutable
    // let mutable_borrow = &mut s;
}

fn mutable_borrow_of_mutable_variable() {
    let mut s = String::from("hello");
    let mutable_borrow = &mut s;

    // this is ok
    println!("The value of mutable_borrow is {}", mutable_borrow);

    mutable_borrow.push_str(" dolly!");
    println!("The value of mutable_borrow is now {}", mutable_borrow);

    println!("The value of s is now {}", s);

    // The above line is ok because mutable_borrow is not used again by the time the line is
    // reached, and so mutable_borrow is already out of scope

    // If however mutable_borrow is used after the line (e.g. in a println, as below):

    // println!("The value of mutable_borrow is now {}", mutable_borrow);

    // then the line previous line:

    // println!("The value of s is now {}", s);

    // no longer compiles, with the error message "cannot borrow `s` as immutable because it
    // is also borrowed as mutable [E0502]"
}

fn can_only_borrow_one_mutable_reference_to_a_mutable_variable() {
    let mut s1 = String::from("a string");
    let t1 = &mut s1;
    let t2 = &mut s1;

    // This error is only flagged when both references t1 and t2 are accessed

    // This is ok
    println!("{t2}");

    // If following line is uncommented:
    // println!("{t1}");

    // then the line:

    // let t2 = &mut s1;

    // is flagged as an error, with the message;
    //   cannot borrow `s1` as mutable more than once at a time [E0499]

}

fn cannot_modify_an_immutable_borrowed_value() {
    let s = String::from("hello");
    change(&s);

    fn change(some_string: &String) {
        // If uncomment the following line, compiler error occurs here,
        // with error message "cannot borrow immutable local variable some_string as mutable"
        //   some_string.push_str(", world");
    }
}

fn mixed_mutability_borrow_ok() {
    let mut x = String::from("a string");

    let immutable_borrow = &x; // immutable borrow
    println!("The value of immutable_borrow is {immutable_borrow}");

    // This compiles as immutable_borrow is out of scope by the time mutable_borrow is initialised
    let mutable_borrow = &mut x; // mutable borrow
    println!("The value of mutable_borrow is {mutable_borrow}");
}

fn cannot_borrow_mutable_if_already_borrowed_as_immutable_1() {
    let mut x = String::from("a string");

    let immutable_borrow = &x; // immutable borrow

    // If println of immutable_borrow is uncommented below, the following statement fails
    // compilation, with error:

    // cannot borrow `x` as mutable because it is also borrowed as immutable [E0502]
    let mutable_borrow = &mut x; // mutable borrow

    // Why? Because if you would have mutable and immutable references of the same variable,
    // then the data of that variable could change through that mutable reference and that
    // could cause a lot of problems.

    // If println of immutable_borrow is commented out, it compiles as immutable_borrow is
    // out of scope by the time mutable_borrow is initialised

    // println!("The value of immutable_borrow is {immutable_borrow}");
    println!("The value of mutable_borrow is {mutable_borrow}");
}

fn cannot_borrow_mutable_if_already_borrowed_as_immutable_2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // cannot borrow `s` as mutable because it is also borrowed as immutable [E0502]
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{r1}, {r2}, and {r3}");
    println!("{r1} and {r2}");
}

fn can_modify_a_borrowed_mutable_value() {
    let mut s = String::from("hello");

    println!("The value of s is: {}", s);

    change(&mut s); // pass a mutable reference

    println!("The new value of s is: {}", s);

    fn change(some_string: &mut String) {
        // can borrow mutable local variable
        some_string.push_str(", world");
    }
}

fn cannot_move_borrowed_mutable_reference() {
    let mut s1 = String::from("a string");
    let t1 = &mut s1;
    println!("{t1}");
    let t2 = s1;

    // Next line creates a compilation error at line
    // let t2 = s1;
    // cannot move out of `s1` because it is borrowed [E0505]

    // println!("{t1}, {t2}")
}

fn cannot_borrow_immutable_local_variable_as_mutable_2() {
    let mut s1 = String::from("a string");
    let t1 = &mut s1;
    println!("{t1}");

    // Cannot borrow immutable local variable `t1` as mutable
    // let t2 = &mut t1;
}

fn borrowing_combos() {
    let mut s1 = String::from("a string");
    let mut t1 = &mut s1;
    println!("{t1}");
    let t2 = &mut t1;

    // (1) print t1 by itself is ok
    // println!("{t1}");

    // (2) print t2 by itself is ok
    // println!("{t2}");

    // (3) print t1 followed by t2 on separate lines
    // printing t1 results in error:
    //   cannot borrow `t1` as immutable because it is also borrowed as mutable [E0502]
    // println!("{t1}");
    // println!("{t2}");

    // (3.1) Same problem if print both values in a single statement
    // println!("{t1}, {t2}");

    // (4) print t2 followed by t1 on separate lines
    // This is fine as t2 is released before t1 is used
    // println!("{t2}");
    // println!("{t1}");

    // (4.1) BUT error if print both values in a single statement
    //   cannot borrow `t1` as immutable because it is also borrowed as mutable [E0502]
    //   i.e. t2 is only released after the statement in which it is last used.
    // println!("{t2}, {t1}");
}

// Same as previous method, but t2 declared as mutable
// Code behaves exactly the same
fn more_borrowing_combos() {
    let mut s1 = String::from("a string");
    let mut t1 = &mut s1;
    println!("{t1}");
    let mut t2 = &mut t1;

    // (1) print t1 by itself is ok
    // println!("{t1}");

    // (2) print t2 by itself is ok
    // println!("{t2}");

    // (3) print t1 followed by t2 on separate lines
    // printing t1 results in error:
    //   cannot borrow `t1` as immutable because it is also borrowed as mutable [E0502]
    // println!("{t1}");
    // println!("{t2}");

    // (3.1) Same problem if print both values in a single statement
    // println!("{t1}, {t2}");

    // (4) print t2 followed by t1 on separate lines
    // This is fine as t2 is released before t1 is used
    // println!("{t2}");
    // println!("{t1}");

    // (4.1) BUT error if print both values in a single statement
    //   cannot borrow `t1` as immutable because it is also borrowed as mutable [E0502]
    //   i.e. t2 is only released after the statement in which it is last used.
    // println!("{t2}, {t1}");
}

fn multiple_scopes() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("The value of r1 is: {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // Cannot find value `r1` in this scope [E0425]
    // println!("The value of r1 and r2 are: {}, {}", r1, r2);
    println!("The value of r2 is: {}", r2);
}

fn main() {
    //basic_string_moves();
    // ownership_and_functions();
    // return_values_and_scope();
    // returning_ownership_of_parameters();
    // string_length_with_borrow();
    // immutable_local();
    // mutable_local();
    // multiple_immutable_borrows_of_mutable_variable_is_ok();
    // multiple_immutable_borrows_of_immutable_variable_is_ok();
    // cannot_borrow_immutable_local_variable_as_mutable_1();
    // can_only_borrow_one_mutable_reference_to_a_mutable_variable();
    // cannot_modify_an_immutable_borrowed_value();
    // mixed_mutability_borrow_ok();
    // cannot_borrow_mutable_if_already_borrowed_as_immutable_1();
    cannot_borrow_mutable_if_already_borrowed_as_immutable_2();
    // can_modify_a_borrowed_mutable_value();
    // cannot_move_borrowed_mutable_reference();
    // cannot_borrow_immutable_local_variable_as_mutable_2();
    // borrowing_combos();
    // more_borrowing_combos();
    // multiple_scopes();
}
