use sayo_parser::parse;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "../../main_encoding.s"
    };

    println!("Attempting to parse: {}", file_path);
    
    // Try reading with different encodings
    let content = match fs::read_to_string(file_path) {
        Ok(s) => {
            println!("✓ Successfully read file as UTF-8");
            s
        }
        Err(e) => {
            println!("✗ Failed to read as UTF-8: {}", e);
            println!("Trying to read as bytes and convert...");
            
            match fs::read(file_path) {
                Ok(bytes) => {
                    // Try to decode as Windows-1252 or Latin-1
                    let decoded = bytes.iter()
                        .map(|&b| b as char)
                        .collect::<String>();
                    println!("✓ Read as bytes, converted to string");
                    decoded
                }
                Err(e2) => {
                    eprintln!("Failed to read file: {}", e2);
                    return;
                }
            }
        }
    };

    println!("File size: {} bytes, {} lines", content.len(), content.lines().count());
    
    match parse(&content) {
        Ok(program) => {
            println!("✓ Successfully parsed {} items", program.items.len());
            
            // Count different types
            let mut directives = 0;
            let mut labels = 0;
            let mut instructions = 0;
            
            for item in &program.items {
                match &item.node {
                    sayo_ast::Item::Directive(_) => directives += 1,
                    sayo_ast::Item::Label(_) => labels += 1,
                    sayo_ast::Item::Instruction(_) => instructions += 1,
                }
            }
            
            println!("  - Directives: {}", directives);
            println!("  - Labels: {}", labels);
            println!("  - Instructions: {}", instructions);
            
            // Show all items
            println!("\nAll {} items:", program.items.len());
            for (i, item) in program.items.iter().enumerate() {
                println!("  [{}] {:?}", i, item);
            }
        }
        Err(e) => {
            eprintln!("✗ Parse error: {:?}", e);
        }
    }
}
