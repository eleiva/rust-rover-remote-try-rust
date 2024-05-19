use std::io::stdin;

fn calculate_fahrenheit(centigrade: i32) -> i32 {
    return (centigrade * 9 / 5) + 32;
}

pub fn c2f() {
    println!("Pls enter a centigrade value: ");

    let mut centigrade = String::new();

    stdin()
        .read_line(&mut centigrade)
        .expect("Did not enter a valid string");

    println!("The value entered was: {centigrade}");

    let _centigrade_int: i32 = centigrade
        .trim()
        .parse()
        .expect("Couldn't parse");

    let _fahrenheit: i32 = self::calculate_fahrenheit(_centigrade_int);

    println!("{_centigrade_int} is {_fahrenheit} in farenheit");
}