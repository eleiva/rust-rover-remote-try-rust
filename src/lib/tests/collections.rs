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
    let mut vec: Vec<String> = vec!["apple".to_string(),
                                    "first".to_string(),
                                    "second".to_string(),
                                    "element".to_string(),
                                    "iris".to_string()];

    for item in vec.iter_mut() {
        if item.starts_with(|c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
            *item = format!("{}-hay", item);
        } else {
            *item = format!("{}-{}ay", &item[1..], item.chars().next().unwrap());
        }
    }

    assert_eq!(vec[0], "apple-hay");
    assert_eq!(vec[1], "irst-fay");
    assert_eq!(vec[2], "econd-say");
    assert_eq!(vec[3], "element-hay");
    assert_eq!(vec[4], "iris-hay");
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' |
                 'A' | 'E' | 'I' | 'O' | 'U')
}

fn convert_to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    if let Some(first_char) = chars.next() {
        if is_vowel(first_char) {
            format!("{}-hay", word)
        } else {
            format!("{}-{}ay", chars.collect::<String>(), first_char)
        }
    } else {
        word.to_string()
    }
}

#[test]
fn solve_pig_latin_another_solution() {
    let vec = vec!["apple", "first", "second", "element", "iris"];

    let pig_latin_words: Vec<String> = vec.iter()
        .map(|&word| convert_to_pig_latin(word))
        .collect();

    assert_eq!(pig_latin_words[0], "apple-hay");
    assert_eq!(pig_latin_words[1], "irst-fay");
    assert_eq!(pig_latin_words[2], "econd-say");
    assert_eq!(pig_latin_words[3], "element-hay");
    assert_eq!(pig_latin_words[4], "iris-hay");
}

fn parse_company_employees_by_department_string(item: &str) -> (String, String) {
    let words: Vec<&str> = item.split_whitespace().collect();

    return (words[1].to_string(), words[3].to_string());
}

#[test]
fn company_employees_by_department() {
    let operations = vec![
        "Add Sally to Engineering".to_string(),
        "Add Edu to Sales".to_string(),
        "Add Nati to Marketing".to_string(),
        "Add Ted to Engineering".to_string(),
        "Add Ramiro to Engineering".to_string(),
        "Add Stella to Marketing".to_string(),
    ];

    let mut organization: HashMap<String, Vec<String>> = HashMap::new();

    for item in operations.iter() {
        let (name, department) = parse_company_employees_by_department_string(item);
        let value = organization.entry(department).or_insert(Vec::new());
        value.push(name);
    }


    let keys: Vec<&String> = organization.keys().collect();
    if let Some(first_key) = keys.first() {
        if let Some(people) = organization.get(*first_key) {
            if first_key.to_string() == "Engineering" {
                assert_eq!("Sally", people.first().unwrap());
                assert_eq!(people.len(), 3)
            }

            if first_key.to_string() == "Sales" {
                assert_eq!("Edu", people.first().unwrap());
                assert_eq!(people.len(), 1)
            }
        }
    }

    assert_eq!(organization.len(), 3);
}