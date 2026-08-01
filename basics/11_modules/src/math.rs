// pub keyword allows access from outside this module
// Without the pub keyword, we cannot access this function from outside the module.
// In Rust, functions are private by default.
pub fn add(a:i32 , b:i32) -> i32 {
    a + b
}
