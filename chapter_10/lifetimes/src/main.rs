
enum LongestLifetime<'a, 'b> {
    X(&'a str),
    Y(&'b str),
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> LongestLifetime<'a, 'b> {
    if x.len() > y.len() {
        LongestLifetime::X(x)
    } else {
        LongestLifetime::Y(y)
    }
}

fn main() {

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());

        match result {
            LongestLifetime::X(a) => println!("{}", a),
            LongestLifetime::Y(b) => println!("b is dead"),
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

}

struct ImportantExcerpt<'a> {
    part: &'a str,
}