use sort::quicksort;
use std::collections::HashMap;
use std::io;

const COMPANY_DEPARTMENTS: [&str; 7] = [
    "Engineering",
    "Sales",
    "Marketing",
    "Dance",
    "PolloFrito",
    "JordanJackson",
    "MemphisGrizzlies",
];

struct Person {
    name: String,
    department: String,
}

impl Person {
    fn clone(&self) -> Person {
        Person {
            name: self.name.clone(),
            department: self.department.clone(),
        }
    }
}

enum Actions {
    Add,
    List,
    None,
}

impl Actions {
    fn unwrap_to_string(&self) -> String {
        match self {
            Actions::Add => return String::from("Add"),
            Actions::List => return String::from("List"),
            Actions::None => return String::from("None"),
        };
    }
}

// struct Department {}

fn main() {
    let mut persons: Vec<Person> = Vec::new();
    create_placeholder_data(&mut persons);

    loop {
        println!("\nWhat do you want to do?");
        let act = get_action();

        match act {
            Actions::Add => {
                let new_person = create_people();
                let mut is_valid_department: bool = false;

                for d in COMPANY_DEPARTMENTS {
                    if new_person.department.ends_with(d) {
                        is_valid_department = true
                    }
                }

                if is_valid_department {
                    persons.push(new_person.clone());
                    println!(
                        "\n{} has been added to {}\n",
                        new_person.name.trim(),
                        new_person.department
                    );
                } else {
                    println!("\nThat is not a valid department\n");
                    continue;
                }
            }
            Actions::List => list_people(&persons),
            Actions::None => continue,
        }
    }
}

fn create_placeholder_data(persons: &mut Vec<Person>) {
    persons.push(
        Person {
            name: String::from("Juan"),
            department: String::from("Sales"),
        },
    );
    persons.push(
        Person {
            name: String::from("Poe"),
            department: String::from("Sales"),
        },
    );
    persons.push(
        Person {
            name: String::from("Messi"),
            department: String::from("Sales"),
        },
    );
    persons.push(
        Person {
            name: String::from("Cristiano"),
            department: String::from("PolloFrito"),
        },
    );
    persons.push(
        Person {
            name: String::from("Azcino"),
            department: String::from("Marketing"),
        },
    );
    persons.push(
        Person {
            name: String::from("Elon"),
            department: String::from("Marketing"),
        },
    );
    persons.push(
        Person {
            name: String::from("Nicolas"),
            department: String::from("Dance"),
        },
    );
    persons.push(
        Person {
            name: String::from("Matajari"),
            department: String::from("Engineering"),
        },
    );
    persons.push(
        Person {
            name: String::from("DrJ"),
            department: String::from("MemphisGrizzlies"),
        },
    );
    persons.push(
        Person {
            name: String::from("Sun Tzu"),
            department: String::from("Engineering"),
        },
    );
}

fn get_action() -> Actions {
    let commands_and_actions: HashMap<u8, Actions> =
        HashMap::from([(1, Actions::Add), (2, Actions::List)]);

    let mut buf = String::new();
    println!("1: Add, 2: List");
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    let buf: u8 = match buf.trim().parse() {
        Ok(num) => num,
        Err(_) => return Actions::None,
    };

    let act_to_return = commands_and_actions.get(&buf).unwrap_or(&Actions::None);

    match act_to_return {
        &Actions::Add => return Actions::Add,
        &Actions::List => return Actions::List,
        &Actions::None => return Actions::None,
    }
}

fn create_people() -> Person {
    let mut name = String::new();
    println!("\nName of the person");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = String::from(&name[..name.len() - 1]);

    let department = list_and_get_department();

    Person { name, department }
}

fn list_people(persons: &Vec<Person>) {
    println!("\nList people in a department (1), list people in the company by department (2)");

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let option: u8 = match option.trim().parse() {
        Ok(num) => num,
        _ => 0,
    };

    if option == 1 || option == 2 {
        if option == 1 {
            let department = list_and_get_department();
            let mut people_to_show: Vec<String> = Vec::new();
            println!("");

            for p in persons.iter() {
                if p.department.ends_with(&department) {
                    people_to_show.push(p.name.clone())
                }
            }

            quicksort(&mut people_to_show);
            println!("Department {} workers", department);
            for (i, worker) in people_to_show.iter().enumerate() {
                println!("{}: {}", i + 1, worker);
            }
        } else {
            let mut people_to_show: HashMap<String, Vec<String>> = HashMap::new();
            println!("");

            for p in persons.iter() {
                for dep in COMPANY_DEPARTMENTS {
                    if p.department.ends_with(dep) {
                        people_to_show.entry(p.department.clone()).or_insert(vec![p.name.clone()]).push(p.name.clone());
                    }
                }
            }

            for (k, v) in &mut people_to_show {
                quicksort(v);
                println!("Department {} workers", {k});
                for (i, p) in v.iter().enumerate() {
                    println!("{}: {}", i + 1, p);
                }
            }
        }
    }
}

fn list_and_get_department() -> String {
    let mut departments = String::new();
    for dep in COMPANY_DEPARTMENTS {
        departments.push_str(dep);
        departments.push(' ');
    }
    println!("\nSelect one department: {}", departments);

    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");

    String::from(&department[..department.len() - 1])
}
