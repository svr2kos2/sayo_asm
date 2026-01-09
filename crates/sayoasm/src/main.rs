use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// Read file with automatic encoding detection (UTF-8, UTF-16 LE/BE)
fn read_file_with_encoding(path: &PathBuf) -> anyhow::Result<String> {
    let bytes = fs::read(path)?;
    
    // Check for BOM and decode accordingly
    if bytes.len() >= 2 {
        // UTF-16 LE BOM
        if bytes[0] == 0xFF && bytes[1] == 0xFE {
            let utf16_data: Vec<u16> = bytes[2..]
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();
            return Ok(String::from_utf16_lossy(&utf16_data));
        }
        // UTF-16 BE BOM
        if bytes[0] == 0xFE && bytes[1] == 0xFF {
            let utf16_data: Vec<u16> = bytes[2..]
                .chunks_exact(2)
                .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                .collect();
            return Ok(String::from_utf16_lossy(&utf16_data));
        }
    }
    
    // Check for UTF-8 BOM
    if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
        return Ok(String::from_utf8_lossy(&bytes[3..]).to_string());
    }
    
    // Default to UTF-8
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

#[derive(Parser)]
#[command(name = "sayoasm")]
#[command(about = "Sayo assembler - assemble .s files into machine code", long_about = None)]
struct Cli {
    /// Input assembly file (.s)
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Output binary file
    #[arg(short = 'o', long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Generate listing file with encoding
    #[arg(short = 'l', long)]
    listing: bool,

    /// Output listing file path (default: <input>.lst)
    #[arg(long, value_name = "FILE")]
    listing_output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Read input file with encoding detection
    let source = read_file_with_encoding(&cli.input)?;

    // Parse assembly
    println!("Parsing {}...", cli.input.display());
    let program = match sayo_parser::parse(&source) {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            std::process::exit(1);
        }
    };

    println!("Found {} items", program.items.len());

    // Assemble
    println!("Assembling...");
    let assembler = sayo_assembler::Assembler::new(source.clone(), program);
    let output = match assembler.assemble() {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Assembly error: {}", e);
            std::process::exit(1);
        }
    };

    println!("Generated {} bytes of machine code", output.machine_code.len());
    println!(
        "Text section: {} bytes, Data section: {} bytes",
        output
            .layout
            .section_sizes
            .get(&sayo_assembler::Section::Text)
            .unwrap_or(&0),
        output
            .layout
            .section_sizes
            .get(&sayo_assembler::Section::Data)
            .unwrap_or(&0)
    );

    // Write binary output
    if let Some(output_path) = cli.output {
        fs::write(&output_path, &output.machine_code)?;
        println!("Wrote binary to {}", output_path.display());
    }

    // Write listing output
    if cli.listing {
        let listing_path = cli.listing_output.unwrap_or_else(|| {
            let mut path = cli.input.clone();
            path.set_extension("lst");
            path
        });
        fs::write(&listing_path, &output.listing)?;
        println!("Wrote listing to {}", listing_path.display());
    }

    Ok(())
}
