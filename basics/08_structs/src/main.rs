struct User {
    name: String,
    age: u16,
    active: bool,
}

fn main() {
    let user = User {
        name: String::from("Nebez"),
        age: 32,
        active: true,
    };

    println!("============================");
    println!();
    println!("User Name : {}", user.name);
    println!("User Age  : {}", user.age);
    println!("Is Active : {}", user.active);
    println!();
    println!("============================");
}
