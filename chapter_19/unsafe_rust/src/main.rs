use std::slice;

// Accessing or Modifying a Mutable Static Variable
static mut COUNTER: u32 = 0; 
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Unsafe Superpowers
// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

fn main() {
    // Dereferencing a Raw Pointer
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    println!("{:?}", r1);
    println!("{:?}", r2);
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Calling an Unsafe Function or Method
    unsafe {
        dangerous()
    }
    
    // Creating a Safe Abstraction over Unsafe Code
    let mut viktor = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut viktor, 3);
    println!("The viktor(vector) left part {:?}", left);
    println!("The viktor(vector) right part {:?}", right);

    // Using extern Functions to Call External Code
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or Modifying a Mutable Static Variable
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Creating a Safe Abstraction over Unsafe Code
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Calling an Unsafe Function or Method
unsafe fn dangerous() {
    println!("I'm dangerous like Simon Bolivar");
}

// Using extern Functions to Call External Code
extern "C" {
    fn abs(input: i32) -> i32;
}

// Mangling is when a compiler changes the name we’ve given a function to a different name that contains
// more information for other parts of the compilation process to consume but is less human readable.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}