fn main() {
    print_labeled_measurement(2147483647, '🚀');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    let labeled_measurement = create_labeled_measurement(value, unit_label);

    println!("The measurement is: {labeled_measurement}");
}

fn create_labeled_measurement(value: i32, unit_label: char) -> String {
    format!("{value}{unit_label}")
}