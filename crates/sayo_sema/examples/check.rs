use sayo_parser::parse;
use sayo_sema::SemanticChecker;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return;
    }
    
    let content = fs::read_to_string(&args[1]).expect("Failed to read file");
    let program = parse(&content).expect("Parse failed");
    
    let mut checker = SemanticChecker::new();
    match checker.check(&program) {
        Ok(_) => println!("Semantic check passed!"),
        Err(errors) => {
            println!("Semantic errors:");
            for e in errors {
                println!("  {}", e);
            }
        }
    }
}
