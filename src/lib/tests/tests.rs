fn _test_clear_screen() {
    print!(" {}[2J", 27 as char);
}

#[test]
#[should_panic(expected = "foo")]
fn test_should_panic() {
    if true {
        panic!("foo");
    } else {
        panic!("abc");
    }
}

#[test]
#[ignore]
fn it_works() -> Result<(), String> {
    if 2 + 3 == 4 {
        return Err(String::from("two plus two does not equal four"))
    }

    Ok(())
}

#[test]
fn also_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}