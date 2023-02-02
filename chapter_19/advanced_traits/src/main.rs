use std::ops::Add;

use advanced_traits::{Millimeters, Meters, Pilot, Human, Wizard};

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    // Default Generic Type Parameters and Operator Overloading
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(3000) + Meters(3), Millimeters(6000));

    // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    Human::fly(&person);
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    <Human as Pilot>::fly(&person);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Using Supertraits to Require One Trait’s Functionality Within Another Trait
    let p = Point { x: 1, y: 3 };
    p.outline_print();

    // Using the Newtype Pattern to Implement External Traits on External Types
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Using Supertraits to Require One Trait’s Functionality Within Another Trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Using the Newtype Pattern to Implement External Traits on External Types
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
