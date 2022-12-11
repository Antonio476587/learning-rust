use std::io;

fn main() {
    println!("\nThis is a degree convertor");
    
    println!("\nSelect the type of degree to convert from");
    let first_type_of_degree: u8 = get_type_of_degree();
    println!("\nSelect the type of degree to convert to");
    let second_type_of_degree: u8 = get_type_of_degree();

    let mut degree_to_convert: f64 = get_degrees();

    if first_type_of_degree == second_type_of_degree {
        println!("{degree_to_convert} will be the same");
    } else {
        if first_type_of_degree != 2 {
            degree_to_convert = to_celsius(first_type_of_degree, degree_to_convert);
        }

        if second_type_of_degree == 2 {
            println!("{degree_to_convert}Cº");
        } else {
            degree_to_convert = from_celsius(second_type_of_degree, degree_to_convert);
            println!("{degree_to_convert}{}º", if second_type_of_degree == 1 { "F" } else { "K" });
        }
    }
}

fn get_type_of_degree() -> u8 {
    println!("1(Fahrenheit), 2(Celsius), 3(Kelvin)");

    loop {
        let mut type_of_degree = String::new();

        io::stdin()
            .read_line(&mut type_of_degree)
            .expect("Failed to read line");
    
        match type_of_degree.trim().parse() {
            Ok(num) => {
                if num > 3 || num < 1 {
                    println!("It has to be less than 4 and greater than 0");
                    continue;
                }
                break num
            },
            Err(_) => {
                println!("It has to be a number");
                continue;
            }
        };
    }

}

fn get_degrees() -> f64 {
    println!("\nInput the degrees:");

    loop {
        let mut degrees = String::new();

        io::stdin()
            .read_line(&mut degrees)
            .expect("Failed to read line");
    
        match degrees.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    }
}

fn to_celsius(type_of_degree_to_convert: u8, degrees: f64) -> f64 {
    if type_of_degree_to_convert == 1 {
        (degrees - 32.0) * 5.0/9.0
    } else {
        degrees - 273.15
    }
}

fn from_celsius(type_of_degree_to_convert: u8, degrees: f64) -> f64 {
    if type_of_degree_to_convert == 1 {
        (degrees * 9.0/5.0) + 32.0
    } else {
        degrees + 273.15
    }
}