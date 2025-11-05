use std::fs::File;
use std::io::ErrorKind;

// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // First shortcut
    let greeting_file_result = File::open("hello_again.txt");

    let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello_again.txt") {
            Ok(file) => file,
            Err(e) => panic!("Problem creating the file: {e:?}"),
        },
        other_error => panic!("Problem opening the file: {:?}", other_error),
    });

    let greeting_file_result = File::open("hello_hello_hello.txt");

    // Second shortcut
    let greeting_file = greeting_file_result.unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_hello_hello_hello.txt")
                .unwrap_or_else(|error| panic!("Problem creating the file: {error:?}"))
        } else {
            panic!("Problem opening the file: {error:?}")
        }
    });
}
