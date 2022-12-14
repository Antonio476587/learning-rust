use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut h: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, c) in "hello!".chars().enumerate() {
        h.entry(c).or_insert(Vec::new()).push(i);
    }

    let mut sum = 0;

    println!("{:?}", h.get(&'l').unwrap());

    for i in h.get(&'l').unwrap() {
        sum += *i;
    }

    for (k, v) in &h {
        println!("{} {:?}", k, v);
    }

    println!("{}", sum);
}
