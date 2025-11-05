use std::fs;
use std::fs::File;
use std::io::{self, Read};

// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
fn main() {
    // calls panic if file isn't there
    let greeting_file = File::open("hello.txt").unwrap();

    // calls panic with specified message if file isn't there
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    fn read_username_from_file(filename: String) -> Result<String, io::Error> {
        let username_file_result = File::open(filename);

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match   username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // There is a difference between what the match expressions do in the read_username_from_file
    // and read_username_from_file_2 functions. Error values that have the ? operator called on
    // them go through the from function, defined in the From trait in the standard library, which
    // is used to convert values from one type into another. When the ? operator calls the from
    // function, the error type received is converted into the error type defined in the return type
    // of the current function. This is useful when a function returns one error type to represent
    // all the ways a function might fail, even if parts might fail for many different reasons.
    fn read_username_from_file_2(filename: &str) -> Result<String, io::Error> {
        let mut username_file = File::open(filename)?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    fn read_username_from_file_4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    let result = read_username_from_file(String::from("hello.txt"));
    println!("{}", result.unwrap());

    let result = read_username_from_file_2("hello.txt");
    println!("{}", result.unwrap());

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}
