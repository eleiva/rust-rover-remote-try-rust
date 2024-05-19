pub fn try_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

#[test]
fn test_option_type() {
    let res: Option<i32> = try_division(2,2);
    assert!(res.is_some());
    assert_eq!(res, Some(1));
}

#[test]
fn test_option_type_none() {
    let res: Option<i32> = try_division(2,0);
    assert!(res.is_none());
    assert_eq!(res, None);
}

#[test]
fn test_option_type_value() {
    let res: Option<i32> = try_division(2,2);
    assert!(res.is_some());
}