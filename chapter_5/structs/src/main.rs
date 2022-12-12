struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Struct tuples
struct Color(i32, i32, i32);
struct Cords(i32, i32, i32);

// Unit structs

struct VenezuelaInTheFIFAWorldCup;

fn main() {
    let user1 = User {
        email: String::from("leo@messirve.com"),
        username: String::from("messirveleo123"),
        active: true,
        sign_in_count: 1,
    };

    // This is the same as follow

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // The difference is the struct update syntax

    let user2 = User {
        email: String::from("carlito@quintana.com"),
        ..user1 // Struct Update Syntax
    };

    println!(
        "\n{}\n{}\n{}\n{}",
        user1.email, user2.email, user1.active, user2.active
    );
    // The user1's username value has been borrowed and cannot be printend

    let white = Color(255, 255, 255);
    let jumanji = Cords(350, -231, 9);

    println!(
        "White: {}{}{} in {}{}{} xyz jumanji",
        white.0, white.1, white.2, jumanji.0, jumanji.1, jumanji.2
    );

    let neverGo = VenezuelaInTheFIFAWorldCup;

}

fn build_user(email: String, username: String) -> User {
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
