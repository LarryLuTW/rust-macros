use func_trace::trace;
use lazy_static::lazy_static;

func_trace::init_depth_var!();

#[trace]
fn foo(n: i32) {
    if n > 0 {
        bar(n - 1)
    }
}

#[trace]
fn bar(n: i32) {
    if n > 0 {
        foo(n - 1);
        foo(n - 2);
    }
}

#[trace]
fn fib(n: u32) -> u32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

// count_digits(50) = 2
// count_digits(300) = 3
fn count_digits(n: u32) -> u32 {
    let mut n = n;

    for i in 1.. {
        n = n / 10;
        if n == 0 {
            return i;
        }
    }

    unreachable!()
}

static VAR: u32 = 100;

lazy_static! {
    static ref FIB5: u32 = fib(5);
}

fn main() {
    // 1. format!
    let str = format!("Hello, I am {}, born in {}", "Larry", 1996);
    println!("{}", str);
    let str = format!("{:04} {:.3}", 42, 12.3456);
    println!("{}", str);
    let str = format!("{a} + {b} = {c}", a = 1, b = 2, c = 1 + 2);
    println!("{}", str);

    // 2. vec!
    let pow_of_10 = vec![1, 10, 100, 1000];
    println!("{:?}", pow_of_10);
    // pow_of_10.push(123);

    let vec = vec![10; 5];
    println!("{:?}", vec);

    // 3. maplit
    use std::collections::{HashMap, HashSet};

    let mut _m = HashMap::new();
    _m.insert("one", 1);
    _m.insert("two", 2);
    _m.insert("three", 3);

    let mut _starbugs = HashSet::new();
    _starbugs.insert("Larry");
    _starbugs.insert("GQSM");
    _starbugs.insert("Luka");
    _starbugs.insert("Smalltown");

    let _m: HashMap<&str, _> = [("one", 1), ("two", 2), ("three", 3)]
        .iter()
        .cloned()
        .collect();

    use maplit::{hashmap, hashset};

    let _m = hashmap! {
        "one"   =>  1,
        "two"   =>  2,
        "three" =>  3,
    };

    let _starbugs = hashset! {"Larry", "Luka", "Smalltown", "GQSM"};

    // 4. unreachable!
    let digits_of_102 = count_digits(102);
    println!("{}", digits_of_102);

    // 5. matches!
    // show expanded
    let o = 'o';
    let is_vowel = matches!(o, 'a' | 'e' | 'i' | 'o' | 'u');
    println!("{}", is_vowel);

    let n = 500;
    let is_matched = matches!(n, 1..=1000);
    println!("{}", is_matched);

    let f = 'f';
    let is_alphabet = matches!(f, 'A'..='Z' | 'a'..='z');
    println!("{}", is_alphabet);

    // 6. serde_json!
    use serde_json::json;

    let larry = json!({
        "name": "Larry Lu",
        "age": 24,
        "phones": [
            "0987654321",
            "07-6071234"
        ]
    });
    println!("first phone number: {}", larry["phones"][1]);

    use serde_json::{from_str, Value};

    let json_str = r#"{ "name": "Annie", "age": 25 }"#;
    let annie: Value = from_str(json_str).unwrap();

    println!("Annie's age: {}", annie["age"]);

    // 7. lazy static
    println!("{}", VAR);
    println!("{}", *FIB5);

    // 8. dbg!
    let a = dbg!(1 + 2 + 3 + 4);
    let _b: u32 = dbg!((1..10).sum());

    let c = dbg!(a * 2 + 1);
    if dbg!(dbg!(c + a) > 10) {
        // do something
    }

    // 9. log!
    use simple_logger::SimpleLogger;
    SimpleLogger::new().init().unwrap();

    use log::{error, info, trace};
    error!("Some error occurs");
    info!("I am Larry");
    trace!("Hello World");

    // 10. func_trace
    let _ = fib(4);
    let _ = foo(2);
}
