#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// The methods defined inside an implementation (impl) block are called associated functions
impl Rectangle {
    // Methods with the same name as properties
    fn width(&self) -> bool {
        self.width > 0
    }
    fn height(&self) -> bool {
        self.height > 0
    }
    // self: &Self is equal to &self, Rust allow to shortand it. https://rust-book.cs.brown.edu/ch05-03-method-syntax.html#defining-methods
    fn can_hold(self: &Self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
    // This is an associated function that is not a method because it doesn't implement self as a parameter
    // it's instead being used as a constructor, it has to be called like this Rectangle::square(u32)
    // I think that this is something like a trait
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// The implementation blocks can be defined separately in multiple blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct  Circle {
    radio: u32,
}

fn main() {
    // Aesthetic purpose
    println!("");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() && rect1.height() {
        println!(
            "The area of the rectangle is {} square pixels.\n",
            area(&rect1)
        );
        println!(
            "The area of the rectangle is {} square pixels.\n",
            rect1.area()
        );
    }

    let circle1 = Circle {
        radio: 10,
    };

    println!("The diameter, the circumference and the area of the circle is, d{}, c{}, a{}\n", diameter(&circle1), circumference(diameter(&circle1)), circle_area(&circle1));

    // Displaying or Debugging
    println!("\n\n rect1 is {:?}", rect1);
    println!("\n\n rect1 is {:#?}\n", rect1);
    dbg!(rect1);

    let debug_variable = 3;

    dbg!(dbg!(30 * debug_variable));

    // Methods with the same type of Struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("\nCan rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // A new Square bro
    println!("\nBaby square {:?}", Rectangle::square(90));

    // Aesthetic purpose
    println!("");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
 
fn diameter(circle: &Circle) -> u32 {
    circle.radio * 2
}

fn circumference(diameter: u32) -> f32 {
    3.14 * diameter as f32
}

fn circle_area(circle: &Circle) -> f32 {
    3.14 * (circle.radio * circle.radio) as f32
}