# Slices

A simple Rust exercise to practice String Slices and References.

## Goal

Build a program to understand how Rust can borrow parts of a value without taking ownership.

## Concepts Learned

- Slices
- String Slice (`&str`)
- Borrowing
- Range syntax

## Notes

### String Slice

A slice is a reference to a part of a value without owning it.

Example:

    let text = String::from("This is Rust language");

    let first_word = &text[0..4];

The original `String` keeps ownership of the data.

### Range Syntax

    &text[start..end]

Examples:

    &text[0..4]  // From index 0 to 3
    &text[8..]   // From index 8 to the end
    &text[..]    // Whole string slice

## Example Output

    ============================

    -1 This
    -2 Rust language
    -3 This is Rust language

    ============================

