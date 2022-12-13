use crate::costumer::eat_at_restaurant;

pub mod hosting;

mod serving {
    pub(crate) fn take_order() {}

    pub(self) fn serve_order() {}

    pub(super) fn take_payment() {}
}