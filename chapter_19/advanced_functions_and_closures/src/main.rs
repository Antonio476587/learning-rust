fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice_and_add(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice_and_add(add_one, 5);

    println!("The answer is: {}", answer);


    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    let cl = returns_closure();
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
