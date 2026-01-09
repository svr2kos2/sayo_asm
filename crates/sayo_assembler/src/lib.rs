mod address;
mod binary;
mod encoder;
mod layout;
mod listing;
mod symbol;

pub use address::{Address, Section};
pub use binary::{BinaryError, HEADER_SIZE, generate_header};
pub use encoder::Encoder;
pub use layout::{Layout, LayoutPass};
pub use listing::Listing;
pub use symbol::{Symbol, SymbolTable};

use sayo_ast::Program;
use std::error::Error;

/// Result type for assembler operations
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Main assembler entry point
pub struct Assembler {
    source: String,
    program: Program,
}

impl Assembler {
    pub fn new(source: String, program: Program) -> Self {
        Self { source, program }
    }

    /// Assemble the program and generate machine code + listing
    pub fn assemble(&self) -> Result<AssemblerOutput> {
        // Step 1: Layout pass - assign addresses to all items
        let mut layout_pass = LayoutPass::new();
        let layout = layout_pass.process(&self.program)?;

        // Step 2: Encode pass - generate machine code
        let mut encoder = Encoder::new(&layout);
        let machine_code = encoder.encode(&self.program)?;

        // Step 3: Generate listing with show-encoding format
        let listing = Listing::generate(&self.source, &self.program, &layout, &machine_code)?;

        Ok(AssemblerOutput {
            machine_code,
            listing,
            layout,
        })
    }
}

/// Output of the assembler
pub struct AssemblerOutput {
    pub machine_code: Vec<u8>,
    pub listing: String,
    pub layout: Layout,
}
