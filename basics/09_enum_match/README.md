## Enums & Match

A simple Rust exercise to practice creating and handling Enums with Pattern Matching.

## Goal

Build a program to understand how Enums represent different states and how Match handles each possible value.

## Concepts Learned

- Enums
- Enum variants
- Creating enum values
- Pattern Matching with match
- Handling different states

## Notes

### Enum

An `enum` is a custom data type that defines a set of possible values.

Example:

    enum UserRole {
        Admin,
        User,
        Guest,
    }

### Creating an Enum Value

Create a value by using the enum name and its variant.

Example:

    let role = UserRole::Admin;

### Match

`match` is used to compare a value against different patterns and execute code based on the matched pattern.

Example:

    match role {
        UserRole::Admin => println!("Full Access"),
        UserRole::User => println!("Limited Access"),
        UserRole::Guest => println!("No Access"),
    }

## Example Output

    ============================

    Role : Admin
    Permission : Full Access

    ============================
