use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Panquecas;

fn main() {
    Panquecas::hello_macro();
}
