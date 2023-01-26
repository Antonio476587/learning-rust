fn main() {
    let some_option_value: Option<i32> = None;
    // Refutable pattern
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
    // Irrefutable pattern
    if let x = 5 {
        println!("{}", x);
    };
}
