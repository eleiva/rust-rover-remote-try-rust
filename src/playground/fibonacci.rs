use std::io::stdin;

pub fn run() {
    println!("Pls enter a number value: ");

    let mut quantity = String::new();

    stdin()
        .read_line(&mut quantity)
        .expect("Did not enter a valid input");


    let quantity: u32 = quantity
        .trim()
        .parse()
        .expect("Couldn't parse");

    let _numbers: [u32; 100] = self::calculate_fibonacci(quantity);

    for e in _numbers.iter().take(1) {
        print!("{}", e);
    }
    for e in _numbers.iter().skip(1) {
        print!(", {}", e);
    }
    println!("");
}

fn calculate_fibonacci(p0: u32) -> [u32; 100] {
    const NUMBER_LIMIT: usize = 100;

    let mut _numbers: [u32; 100] = [0; NUMBER_LIMIT];

    let mut x = 0;

    while x < p0 {
        if x == 0 {
            _numbers[0] = 0;
        } else if x == 1 {
            _numbers[1] = 1;
        } else {
            _numbers[x as usize] = _numbers[(x - 1) as usize] + _numbers[(x - 2) as usize];
        }
        x = x + 1;
    }

    return _numbers;
}