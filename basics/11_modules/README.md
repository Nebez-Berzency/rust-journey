## Modules

A simple Rust exercise to practice modules and public functions.

## Problem

When a project grows, keeping all code in one file becomes difficult to manage.

## Solution

Split large files into smaller pieces, and move functions into other files.

## Concepts Learned

- Modules
- Creating modules
- mod keyword
- pub keyword
- Private and public functions
- Accessing functions from another module

## Notes

### Module

A module is a way to organize code into separate files and groups.

Example:

    mod math;

### Public Functions

By default, functions in Rust are private.

Use `pub` keyword to make a function accessible from outside the module.

Example:

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

### Using Modules

Access functions from a module using the module name.

Example:

    let result = math::add(10, 20);

## Project Structure

    src/
    ├── main.rs
    └── math.rs

## Example Output

    ========================

    The result is: 30

    ========================
