use std::io;

fn main() {
    
    println!("Insert a fibonacci number bro");

    let fib_number: i64 = get_fib_number();

    // The first function 'f1' is inefficient with numbers above 40
    // If you want to calculate numbers biggers than 40, comment the next line
    println!("{}", fi1(fib_number));
    println!("{}", fi2(fib_number));
    println!("{}", fi3(fib_number));

}

fn get_fib_number() -> i64 {
    loop {

        let mut fib_number = String::new();

        io::stdin()
            .read_line(&mut fib_number)
            .expect("Failed to read line");

        match fib_number.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    }
}

// Fibonacci sequence formula
fn fi1(number: i64) -> i64{

    if number == 0 || number == 1{
        return number
    }
    
    return fi1(number - 1) + fi1(number - 2)

}

// Binet formula
fn fi2(number: i64) -> f32 {

    if number == 0 || number == 1{
        return (number as f32).into()
    }

    return 1.0/5.0_f32.sqrt() * ((power((1.0 + 5.0_f32.sqrt()) / 2.0, number)) - (power((1.0 - 5.0_f32.sqrt()) / 2.0, number)))
}

fn fi3(number: i64) -> f32 {
    
    if number == 0 || number == 1{
        return (number as f32).into()
    }

    // 1.61803398875 is the golden ratio or aureal number (φ or Φ )
    return (power(1.61803398875, number) - (power(1.0 - 1.61803398875, -number))) / 5.0_f32.sqrt()
}

fn power(number_to_raise: f32, power: i64) -> f32 {
    let mut changing_power = power - 1;
    let mut number_raised = number_to_raise;

    while changing_power > 0 {
        number_raised = number_raised * number_to_raise;

        changing_power = changing_power - 1;
    }

    return number_raised
}

