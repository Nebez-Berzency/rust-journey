// Ownership
fn print_user_color(color: String) {
    println!("User Color : {color}");
}

// Borrowing
fn print_user_name(name: &str) {
    println!("User Name : {name}");
}

fn main() {

    let user_color = String::from("Blue");
    let user_name = String::from("Nabaz");

    // Ownership is moved to the function
    print_user_color(user_color);

    // Borrow the value without taking ownership
    print_user_name(&user_name);

    // Compile Error:
    // user_color is no longer available after the move.
    // println!("{user_color}");
}
