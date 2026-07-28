enum UserRole {
    Admin,
    User,
    Guest,
}

fn main() {

    let status = UserRole::Admin;

    match status {
        UserRole::Admin => {
            println!("Role: Admin");
            println!("Permission: Full Access");
        },

        UserRole::User => {
            println!("Role: User");
            println!("Permission: Limited Access");
        },

        UserRole::Guest => {
            println!("Role: Guest");
            println!("Permission: No Access");
        }
    }
}
