use sayo_ast::{generate_instruction_markdown, generate_register_markdown};
use std::fs;
use std::path::Path;

fn main() {
    // Generate instruction documentation
    let instr_md = generate_instruction_markdown();
    let instr_path = Path::new("docs/instructions.md");
    
    // Create docs directory if it doesn't exist
    if let Some(parent) = instr_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create docs directory");
    }
    
    fs::write(instr_path, instr_md).expect("Failed to write instructions.md");
    println!("✓ Generated instructions.md");
    
    // Generate register documentation
    let reg_md = generate_register_markdown();
    let reg_path = Path::new("docs/registers.md");
    fs::write(reg_path, reg_md).expect("Failed to write registers.md");
    println!("✓ Generated registers.md");
    
    println!("\nDocumentation generated successfully!");
    println!("  - {}", instr_path.display());
    println!("  - {}", reg_path.display());
}
