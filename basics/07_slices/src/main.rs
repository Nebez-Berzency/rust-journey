fn main() {

    let text = String::from("This is Rust language");


    // Borrow a part of the String without taking ownership
    let first_word = &text[0..4];

    // Slice from index 8 to the end of the String
    let from_middle_to_end = &text[8..];

    // Borrow the whole String as a slice
    let whole_sentence = &text[..];


    println!("============================");
    println!();

    println!("-1 {first_word}");
    println!("-2 {from_middle_to_end}");
    println!("-3 {whole_sentence}");

    println!();

    println!("============================");
}
