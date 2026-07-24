# Ownership

A simple Rust exercise to practice Ownership , Move, Copy and Clone.

## Goal

Build a program to understand how Rust manages ownership when values are moved, coped or cloned.

## Consepts Learned 
 - Ownership
 - Move semantics
 - Copy trait
 - Clone method
 - String Slice (`&str`)

 ## Notes

### Move

`String` values own their data.

```Rust
let user_name = String::from("Nabaz");
let new_user = user_name;
```
After the move, 'user_name' is no longer valid.

```Rust
println!("{user_name}"); // /Compile error
```

### Copy

Primitive types implement the `Copy` trait.

```rust
let age = 32;
let new_age = age;
```

Both variables remain valid.

---

### String Slice

String literals (`&str`) are copied instead of moved.

```rust
let location = "Mahabad";
let new_location = location;
```

---

### Clone
`clone()` creates a new owned copy when needed.

```rust
let cloned_user = new_user.clone();
```

## Example Output

```text
============================

User : Nabaz
Age : 32
Location : Mahabad

============================

Cloned User : Nabaz

============================
```
