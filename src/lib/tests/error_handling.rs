#[test]
fn test_error_handling() {
    use std::fs::File;

    let greeting_file_result = File::open("Cargo.toml");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

#[test]
fn matching_diff_errors() {
    use std::fs::File;
    use std::io::ErrorKind;

    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

use std::io;
use std::fs::File;
use std::io::Read;

fn _read_username_from_file(filename: String) -> Result<String, io::Error> {
    let username_file_result = File::open(filename);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

#[test]
fn read_file() {
    let res = read_username_from_file_3("hello.txt".to_string());
    match res {
        Ok(file) => println!("{}", file),
        Err(e) => println!("{}", e),
    }
}

fn _read_username_from_file_2(filename: String) -> Result<String, io::Error> {
    let mut username_file = File::open(filename)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3(filename: String) -> Result<String, io::Error> {
    std::fs::read_to_string(filename)
}

#[test]
fn test_error_handling_with_option() {
    let guess: i32 = match "2".trim().parse::<i32>() {
        Ok(num) => num + 1,
        Err(err) => panic!("ERROROROROROR {}", err),
    };

    assert_eq!(guess, 3)
}

