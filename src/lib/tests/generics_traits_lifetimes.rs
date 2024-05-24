#[test]
fn find_largest_value_in_stack() {
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
fn find_largest_value_in_heap() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    assert_eq!(largest, &100);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[test]
fn test_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[test]
fn add_functionality_to_a_generic_type() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {} - p.y = {}", p.x(), p.y);
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

#[test]
fn test_generics_2() {
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

pub trait Summary {
    fn summarize(&self) -> String;

    fn default_impl(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn default_impl(&self) -> String {
        String::from("(Read more... NewsArticle)")
    }
}

pub struct NewsArticle2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle2 {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn default_impl(&self) -> String {
        String::from("(Read more... NewsArticle)")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[test]
fn test_traits() {
    let t1 = Tweet {
        username: "edu".to_string(),
        content: "my content".to_string(),
        reply: false,
        retweet: true,
    };
    let na1 = NewsArticle {
        headline: "headline of article".to_string(),
        location: "chivilcoy".to_string(),
        author: "maru".to_string(),
        content: "soome content".to_string(),
    };

    assert_eq!("edu: my content", t1.summarize());
    assert_eq!("(Read more...)", t1.default_impl());
    assert_eq!("headline of article, by maru (chivilcoy)", na1.summarize());
    assert_eq!("(Read more... NewsArticle)", na1.default_impl());
}

fn accept_a_specific_kind(item: &impl Summary) -> String {
    format!("{} - in the fn", item.summarize())
}

fn same_as_previous_function<T: Summary>(item: &T) -> String {
    format!("{} - in the fn", item.summarize())
}

fn accepting_2_params_with_new_sintax(item1: &impl Summary, item2: &impl Summary) -> String {
    format!("{}-{}", &item1.summarize()[0..2], &item2.summarize()[0..2])
}

#[test]
fn test_trait_as_parameter() {
    let t1 = Tweet {
        username: "edu".to_string(),
        content: "my content".to_string(),
        reply: false,
        retweet: true,
    };

    let na1 = NewsArticle {
        headline: "headline of article".to_string(),
        location: "chivilcoy".to_string(),
        author: "maru".to_string(),
        content: "soome content".to_string(),
    };

    assert_eq!("edu: my content - in the fn", accept_a_specific_kind(&t1));
    assert_eq!(
        "headline of article, by maru (chivilcoy) - in the fn",
        accept_a_specific_kind(&na1)
    );

    assert_eq!(
        "edu: my content - in the fn",
        same_as_previous_function(&t1)
    );
    assert_eq!(
        "headline of article, by maru (chivilcoy) - in the fn",
        same_as_previous_function(&na1)
    );

    assert_eq!("ed-he", accepting_2_params_with_new_sintax(&t1, &na1));
}

pub trait Display {
    fn show(&self) -> String;
}

impl Display for NewsArticle {
    fn show(&self) -> String {
        format!("the content {}", self.content)
    }
}

impl Display for Tweet {
    fn show(&self) -> String {
        format!("the content tweet -> {}", self.content)
    }
}

pub fn notify<T: Summary + Display>(item: &T) -> String {
    format!("{} - {}", item.show(), accept_a_specific_kind(item))
}
pub fn notify2(item: &(impl Summary + Display)) -> String {
    format!("{} - {}", item.show(), accept_a_specific_kind(item))
}

#[test]
fn test_multiple_traits() {
    let t1 = Tweet {
        username: "edu".to_string(),
        content: "my content".to_string(),
        reply: false,
        retweet: true,
    };

    assert_eq!(
        "the content tweet -> my content - edu: my content - in the fn",
        notify(&t1)
    );
    assert_eq!(
        "the content tweet -> my content - edu: my content - in the fn",
        notify2(&t1)
    );
}

fn _trait_bound_example_with_references<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone,
{
    0
}

fn returns_summarizable_but_limited_to_only_one_type(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    }
}

#[test]
fn retrieve_a_specific_type() {
    let t = returns_summarizable_but_limited_to_only_one_type(true);

    assert_eq!("Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)", t.summarize());
}

#[test]
fn borrow_checker_and_lifetimes() {
    let r;
    let y;

    {
        let x = 5;
        r = x;
        y = &x;

        println!("y: {}", y);
    }

    println!("r: {}", r);
}

fn longest<'a>(s1: &'a str, _s2: &'a str) -> &'a str {
    s1
}

fn longest2(_s1: &str, _s2: &str) -> String {
    let result = String::from("really long string");
    result
}

#[test]
fn first_time_around_lifetime() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());

    assert_eq!("long string is long",result);
    assert_eq!("really long string", longest2(&string1, &string2));
}

