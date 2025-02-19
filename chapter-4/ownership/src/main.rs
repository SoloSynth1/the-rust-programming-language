use std::collections::HashMap;


fn fib(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    if !memo.contains_key(&n) {
        let result = fib(n - 1, memo) + fib(n - 2, memo);
        memo.insert(n, result);
    }
    memo[&n]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", &s);

    let mut memo: HashMap<u128, u128> = HashMap::new();
    memo.insert(1, 1);
    memo.insert(2, 1);
    println!("{}", fib(100, &mut memo));
    println!("The first word of {} is {}.", &s, &first_word(&s));
    s.clear();

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{}", slice == &[2, 3]);
}
