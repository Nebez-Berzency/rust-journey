fn main() {
    
    let user_name = String::from("Nabaz");
    let age = 32;
    let location = "Mahabad";

    // Ownership is moved from user_name to new_user
    // user_name is no longer available after this move
    let new_user = user_name;
    let new_age = age;
    let new_location = location;

    // Creates a new owned copy of new_user
    // Both variables contain the same value
    let cloned_user = new_user.clone();

    println!("============================");
    println!();
    println!("User : {new_user}");
    println!("Age : {new_age}");
    println!("Location : {new_location}");
    println!();
    println!("============================");
    println!();
    println!("Cloned User : {cloned_user}");
    println!();
    println!("============================");
}
