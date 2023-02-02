fn main() {
    // Using the Newtype Pattern for Type Safety and Abstraction
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| ())
    }

    // The Never Type that Never Returns is !

    for number in 1..=10 {
        if number % 2 == 0 {
            continue; // is ! 
        }
        println!("{number}");
    }
    
    fn bar() -> ! {
        panic!();
    }
    
    // Dynamically Sized Types and the Sized Trait
    // https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
    fn generic<T: ?Sized>(t: &T) {
    }
    

}
