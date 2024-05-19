
#[test]
fn empty_test_lib() {
    assert!(true)
}

//cargo test --color=always --package hello_remote_world --lib tests::test_lib::test_match -- --show-output
#[test]
fn test_match() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => assert_eq!(max, 3u8),
        _ => (),
    }
}

#[test]
fn test_dont_match() {
    let config_max: Option<i32> = None;
    let mut x: bool = false;
    match config_max {
        Some(_max) => (),
        _ => x = true,
    }

    assert!(x)
}

#[test]
fn test_let_if_match() {
    let config_max = Some(3u8);
    if let Some(_max) = config_max {
        assert_eq!(_max, 3u8);
        assert_ne!(_max, 4u8);
    }
}

#[test]
fn test_let_if_not_match() {
    let config_max = Some(3u8);
    if let None = config_max {
        println!("The maximum is configured to be");
    }

    assert!(true);
}

#[test]
fn reference_and_dereference() {
    let x = 10;        // Un entero simple
    let ref_x = &x;    // Una referencia a `x`

    // Para acceder al valor de `x` a través de `ref_x`, necesitas desreferenciar `ref_x`:
    let y = *ref_x;    // `y` ahora es igual a 10

    println!("El valor de ref_x es: {}", ref_x);
    println!("El valor de y es: {}", y);
}

#[test]
fn test_vectors() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let _third: Option<&i32> = v.get(2);
    match _third {
        Some(_third) => println!("The third element is {_third}"),
        None => println!("There is no third element."),
    }
}

#[test]
fn test_retrieve_valid_value_from_vector() {
    let v = vec![1, 2, 3, 4, 5];
    let _third: Option<&i32> = v.get(2);
    match _third {
        Some(_third) => assert_eq!(_third, &3),
        None => println!("There is no third element."),
    }
}

#[test]
fn test_retrieve_an_invalid_value_from_vector() {
    let v = vec![1, 2, 3, 4, 5];
    let _third: Option<&i32> = v.get(10);
    match _third {
        Some(_third) => (),
        None => assert!(true),
    }
}

#[test]
fn panic_using_a_borrowed_element() {
    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    v.push(6);

    //println!("The first element is: {first}");
}

#[test]
fn iterate_over_a_vector() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    assert_eq!(v, vec!(150,82,107));
}

#[test]
fn test_string_1 () {
}

#[test]
fn get_slice() {
    let s = String::from("foo");

    assert_eq!("foo", s);
}
#[test]
fn make_ownership_fail() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    //println!("{}", field_name);
}

#[test]
fn updating_hashmaps() {
    use std::collections::HashMap;

    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("Blue"), 33);
    scores.insert(String::from("Blue"), scores.get("Blue").copied().unwrap_or(0) + 10);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}