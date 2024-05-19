use std::collections::HashMap;

#[test]
fn find_median() {
    let mut vec: Vec<u32> = vec![10, 8, 4, 5, 3, 2, 1];

    let median_position: usize = vec.len() / 2;

    vec.sort();
    // println!("{:?}", vec);

    assert_eq!(vec[median_position], 4);
}

#[test]
fn find_mod() {
    let vec: Vec<u32> = vec![10, 4, 4, 3, 3, 4, 1];
    let mut map = HashMap::new();

    for item in vec {
        *map.entry(item.to_string()).or_insert(0) += 1;
    }

    println!("{:?}", map);

    assert_eq!(*map.get("4").unwrap(), 3);
}

#[test]
fn solve_pig_latin() {
    let vec: Vec<u32> = vec![10, 4, 4, 3, 3, 4, 1];
    let mut map = HashMap::new();

    for item in vec {
        *map.entry(item.to_string()).or_insert(0) += 1;
    }

    println!("{:?}", map);

    assert_eq!(*map.get("4").unwrap(), 3);
}