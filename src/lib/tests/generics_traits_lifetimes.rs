#[test]
fn find_largest_value_in_stack () {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    assert_eq!(largest, 100);
}
#[test]
fn find_largest_value_in_heap () {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    assert_eq!(largest, &100);
}

#[derive(Debug)]
struct Fop<T> {
    x: T
}
#[test]
fn test_generics() {
    let num_1 = Fop {
        x: 33
    };

    let num_2 = Fop {
        x: 2.3
    };

    println!("{:?}", num_1);
    println!("{:?}", num_2 );
}