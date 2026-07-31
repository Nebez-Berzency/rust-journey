use std::fs;

fn main() {

    let text_result = fs::read_to_string("file.txt");

    match text_result
    {
        Ok(content) => {
            println!("\n========== File Content ==========\n");
            println!("{content}");
            println!("===================================\n");

        }

        Err(error) => {
            println!("\n========== File Error ==========\n");
            println!("{error}");
            println!("\n===================================\n");
        }
    }
}
