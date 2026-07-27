## Structs

A simple Rust exercise to practice creating and using Structs.

## Goal

Build a program to understand how Struct group related data into a single type.

## Concepts Learned 

- Structs 
- Struct fields
- Creating instances
- Accessing fields with dot syntax

## Notes

### Struct

A `struct` is a custom data type that groups related values together.

Example:

    struct User {
        name: String,
        age : u8,
        active: bool
    }

### Creating an instance

Create a new instance by assigning values to each field.

Example:
    
    let user = User {
        name : String::from("Nebez"),
        age: 32,
        active:true
    };

### Accessing fields

Use dot syntax to access struct fields.


Examples:

    user.name
    user.age
    user.active



## Example Output

    ============================

    User Name : Nebez
    User Age  : 32
    Is Active : true

    ============================



