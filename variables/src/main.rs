#![allow(unused)]
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The valuie of y in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");

    // Floats (inference and static)
    
    let a = 2.0; // f64

    let b: f32 = 3.0; // f32

    println!("{a} or {b}");

    // Characters

    let c = "z";
    let z: char = 'ℤ'; // Literal characters go with single quotes ''
    let heart_eyed_cat = '😻';
    
    println!("c = {c}, z = {z}, crazy_char_emoji = {heart_eyed_cat}");
    
    // // Compund types        

    // Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 4.6, 1);

    let (h, i, j) = tup;

    let i = tup.1;

    println!("(h) {h}, tup.1 {i}");

    let unit_tup = ();

    // Arrays

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    
    let brray = [3; 5];
}
