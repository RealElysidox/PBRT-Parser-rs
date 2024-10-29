use PBRT_Parser_rs::parser::{parse_file, Attribute};

fn main() {
    // Load and parse the PBRT file contents
    let input = std::fs::read_to_string("scene-v4.pbrt").expect("Failed to read the file");

    match parse_file(&input) {
        Ok((_, attributes)) => {
            for attr in attributes {
                println!("{:?}", attr);
            }
        }
        Err(e) => println!("Failed to parse the file: {:?}", e),
    }
}