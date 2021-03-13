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

// #[trace(logging)]
#[trace]
fn fib(n: u32) -> u32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

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

lazy_static! {
    static ref FIB5: u32 = fib(5);
}

fn main() {
    // 1. format!
    let str = format!("Hello, I am {}, born in {}", "Larry", 1996);
    println!("{}", str);
    let str = format!("{a} + {b} = {c}", a = 1, b = 2, c = 1 + 2);
    println!("{}", str);
    let str = format!("{:04} {:.3}", 42, 12.3456);
    println!("{}", str);

    // 2. vec!
    let vec = vec![1, 2, 3];
    println!("{:?}", vec);
    let vec = vec![0; 5];
    println!("{:?}", vec);

    // 3. unreachable!
    let digits_of_102 = count_digits(102);
    println!("{}", digits_of_102);

    // 4. matches!
    // show expanded
    let o = 'o';
    let is_matched = matches!(o, 'a' | 'e' | 'i' | 'o' | 'u');
    println!("{}", is_matched);

    let n = 500;
    let is_matched = matches!(n, 1..=1000);
    println!("{}", is_matched);

    let f = 'f';
    let is_matched = matches!(f, 'A'..='Z' | 'a'..='z');
    println!("{}", is_matched);

    // 5. serde_json!
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

    let annie = json!({
        "name": format!("{} {}", "Annie", "Wang"),
        "age": 24 + 1,
    });
    println!("Annie's age: {}", annie["age"]);

    // 6. lazy static
    println!("{}", *FIB5);

    // 7. maplit
    // https://stackoverflow.com/questions/28392008/more-concise-hashmap-initialization
    use std::collections::HashMap;
    let _m: HashMap<&str, _> = [("one", 1), ("two", 2), ("three", 3)]
        .iter()
        .cloned()
        .collect();

    use maplit::hashmap;
    let _m = hashmap! {
        "one"=>1,
        "two"=> 2,
        "three" => 3,
    };

    use std::collections::HashSet;
    let _s: HashSet<&str> = ["Larry", "Luka", "Smalltown", "GQSM"]
        .iter()
        .cloned()
        .collect();

    use maplit::hashset;
    let _s = hashset! {"Larry", "Luka", "Smalltown", "GQSM"};

    // 8. dbg!
    let a = 10;
    dbg!(a);
    dbg!(a * 2 + 1);

    let b = dbg!(a * 2 + 1);
    if dbg!(b + a) > 10 {
        // do something
    }

    // 9. log!
    use simple_logger::SimpleLogger;
    SimpleLogger::new().init().unwrap();

    use log::{error, info};
    error!("Some error occurs");
    info!("Hello World");
    info!("I am Larry");

    // 10. func_trace
    let _ = fib(4);
    let _ = foo(2);
}
