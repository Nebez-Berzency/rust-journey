# Borrowing

A simple Rust exercise to practice Borrowing and References.

## Goal

Build a program to understand how Rust allows using values without taking ownership.

## Concepts Learned

- Ownership transfer
- Borrowing
- References (`&`)
- String slices (`&str`)

## Notes

### Ownership

Passing a `String` to a function moves ownership.

```rust
fn print_user_color(color: String) {
    println!("User Color : {color}");
}
```

After moving ownership, the original variable is no longer available.

### Borrowing

Borrowing allows a function to use a value without taking ownership.

```rust
fn print_user_name(name: &str) {
    println!("User Name : {name}");
}
```

The original value remains valid after borrowing.

## Example Output

```text
User Color : Blue
User Name : Nabaz
```

