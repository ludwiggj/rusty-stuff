// Taken from https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let mut x = 0;
    println!("The value of x is: {x}");
    x = 1;
    println!("The value of x is: {x}");

    let x = 5;

    println!("The value of x is: {x}");

    let x = x + 1;

    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len(); // type of spaces is now usize

    let mut spacez = "    ";
    // spacez = spacez.len(); // this line would cause a compile-time error - type mismatch
}