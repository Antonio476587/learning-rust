#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Caracas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Caracas)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let six = match six {
        None => None,
        Some(i) => {
            println!("{}", i);
            Some(i)
        }
    };

    // Catch-all and _ patterns

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // This cover every posible value
        _ => reroll(), // is a special pattern that matches any value and does not bind to that value, so you don't have to use it like with 'other'
        _ => (), // This means that we aren't going to use neither the value or a function to handle
    }

    // This is a sample of ownership with enums

    let opt: Option<String> = Some(String::from("Hello world"));

    // opt became &opt, so it is pushed down as a reference
    // https://doc.rust-lang.org/reference/patterns.html#binding-modes
    // https://rust-book.cs.brown.edu/ch06-02-match.html#how-matches-interact-with-ownership
    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);

    // if let and else control flow

    let coin = Coin::Quarter(UsState::Caracas);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(n: i32) {}
fn reroll() {}
