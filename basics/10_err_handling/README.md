## Error Handling

A simple Rust exercise to prictice error handling using `Result` and `match`.

## Gaol

Build a program that reads a text file and handles both success and failure

## Concepts Learned

- Result
- Ok
- Err
- match
- Reading files with `sts::fs`

## Notes

### Result

`Result` is an enum that represents either success or failure.

Example:
    
    Result<T,E>

- `Ok(T)` contains the successful value.
- `Err(E)` contains the error.

### Reading a File

Use `std::fs::read_to_string()` to read a text file.

Example:
    
    let text_result = fs::read_to_string(file.txt);

### Handling Result

Use `match` to handle both possible outcomes.

Example:

    match text_result {
        Ok(content) => println!("{content}");
        Err(error) => println!("{error}");
    }


## Example Output

    ========== File Content ==========

    Hello Rust!

    ================================

## Example Error

    ========== Read Error ==========

    No such file or directory (os error 2)

    ===============================







