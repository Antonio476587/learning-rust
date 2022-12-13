pub mod breakfast;

pub mod lunch;

pub use lunch::*;

// Making an implementation block for Lunch struct outside his native module
impl Lunch {
    pub fn summer(kind: &str) -> Lunch {
        Lunch {
            soap: String::from(kind),
        }
    }
}

fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}

fn cook_order() {}
