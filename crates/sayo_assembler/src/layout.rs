use crate::address::{Address, Section};
use crate::binary::HEADER_SIZE;
use crate::symbol::SymbolTable;
use sayo_ast::{Directive, Item, Program};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LayoutError {
    #[error("Local label '{0}' defined without a global label")]
    LocalLabelWithoutGlobal(String),

    #[error("Unsupported directive: {0}")]
    UnsupportedDirective(String),

    #[error("Invalid alignment value: {0}")]
    InvalidAlignment(u32),
}

/// Layout information for the program
#[derive(Debug, Clone)]
pub struct Layout {
    /// Symbol table with all labels and their addresses
    pub symbols: SymbolTable,
    /// Address for each item in the program (by index)
    pub item_addresses: Vec<Address>,
    /// Global scope for each item (for local label resolution)
    pub item_scopes: Vec<Option<String>>,
    /// Final size of each section
    pub section_sizes: HashMap<Section, Address>,
}

impl Layout {
    pub fn new(symbols: SymbolTable, item_addresses: Vec<Address>, item_scopes: Vec<Option<String>>) -> Self {
        Self {
            symbols,
            item_addresses,
            item_scopes,
            section_sizes: HashMap::new(),
        }
    }
    
    /// Get the address where the text section starts (after header)
    pub fn text_section_start(&self) -> Address {
        HEADER_SIZE
    }
    
    /// Get the address where the data section starts
    pub fn data_section_start(&self) -> Address {
        HEADER_SIZE + self.section_sizes.get(&Section::Text).copied().unwrap_or(0)
    }
}

/// Layout pass - assigns addresses to all items
pub struct LayoutPass {
    symbols: SymbolTable,
    item_addresses: Vec<Address>,
    item_scopes: Vec<Option<String>>,
    current_section: Section,
    /// Text section address (starts after header)
    text_addr: Address,
    /// Data section address (starts after text section)
    data_addr: Address,
}

impl LayoutPass {
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            item_scopes: Vec::new(),
            item_addresses: Vec::new(),
            current_section: Section::Text,
            // Text section starts after the header
            text_addr: HEADER_SIZE,
            // Data section address is determined after text section is complete
            data_addr: 0,
        }
    }

    fn current_addr(&self) -> Address {
        match self.current_section {
            Section::Text => self.text_addr,
            Section::Data => self.data_addr,
        }
    }

    fn advance_addr(&mut self, delta: Address) {
        match self.current_section {
            Section::Text => self.text_addr += delta,
            Section::Data => self.data_addr += delta,
        }
    }

    fn set_addr(&mut self, addr: Address) {
        match self.current_section {
            Section::Text => self.text_addr = addr,
            Section::Data => self.data_addr = addr,
        }
    }

    fn align_addr(&mut self, alignment: u32) -> Result<(), LayoutError> {
        if alignment == 0 {
            return Err(LayoutError::InvalidAlignment(alignment));
        }
        let current = self.current_addr();
        let aligned = (current + alignment - 1) & !(alignment - 1);
        self.set_addr(aligned);
        Ok(())
    }

    pub fn process(&mut self, program: &Program) -> Result<Layout, LayoutError> {
        // Assign addresses to all items
        // Text section starts at HEADER_SIZE
        // Data section starts immediately after text section
        
        for (idx, item) in program.items.iter().enumerate() {
            let item_addr = self.current_addr();
            // Record current global scope for this item
            self.item_scopes.push(self.symbols.current_global().cloned());
            self.item_addresses.push(item_addr);

            match &item.node {
                Item::Label(name) => {
                    if name.starts_with('.') {
                        // Local label
                        self.symbols
                            .define_local(name.clone(), item_addr)
                            .map_err(|_| LayoutError::LocalLabelWithoutGlobal(name.clone()))?;
                    } else {
                        // Global label
                        self.symbols.define_global(name.clone(), item_addr);
                    }
                }

                Item::Instruction(instr) => {
                    // Get instruction length from metadata
                    let metadata = instr.mnemonic.metadata();
                    self.advance_addr(metadata.length as Address);
                }

                Item::Directive(dir) => {
                    self.process_directive(dir, idx)?;
                }
            }
        }

        // Calculate final section sizes
        // Text size is relative to HEADER_SIZE
        let text_size = self.text_addr - HEADER_SIZE;
        // Data size is relative to data section start
        let data_start = HEADER_SIZE + text_size;
        let data_size = if self.data_addr >= data_start {
            self.data_addr - data_start
        } else {
            0
        };
        
        let mut section_sizes = HashMap::new();
        section_sizes.insert(Section::Text, text_size);
        section_sizes.insert(Section::Data, data_size);

        let mut layout = Layout::new(self.symbols.clone(), self.item_addresses.clone(), self.item_scopes.clone());
        layout.section_sizes = section_sizes;

        Ok(layout)
    }

    fn process_directive(&mut self, dir: &Directive, _item_idx: usize) -> Result<(), LayoutError> {
        match dir {
            Directive::Text => {
                self.current_section = Section::Text;
            }
            Directive::Data => {
                self.current_section = Section::Data;
                // Initialize data section address based on where text section ends
                if self.data_addr == 0 {
                    // Data section starts immediately after text section (no overhead)
                    self.data_addr = self.text_addr;
                }
            }
            Directive::Section(name) => {
                // Handle .section .rodata etc
                if name.contains("rodata") {
                    self.current_section = Section::Data;
                    if self.data_addr == 0 {
                        self.data_addr = self.text_addr;
                    }
                }
            }
            Directive::Align(n) => {
                self.align_addr(*n)?;
            }
            Directive::P2align(n) => {
                // .p2align n means align to 2^n bytes
                let alignment = 1u32 << n;
                self.align_addr(alignment)?;
            }
            Directive::Byte(values) => {
                self.advance_addr(values.len() as Address);
            }
            Directive::Word(values) | Directive::Short(values) => {
                let size = values.len() * 2;
                self.advance_addr(size as Address);
            }
            Directive::Long(values) => {
                let size = values.len() * 4;
                self.advance_addr(size as Address);
            }
            Directive::Quad(values) => {
                let size = values.len() * 8;
                self.advance_addr(size as Address);
            }
            Directive::Ascii(s) => {
                let size = s.len();
                self.advance_addr(size as Address);
            }
            Directive::Asciz(s) => {
                // .asciz adds a null terminator
                let size = s.len() + 1;
                self.advance_addr(size as Address);
            }
            Directive::Zero(count) => {
                let size = *count as usize;
                self.advance_addr(size as Address);
            }
            Directive::Skip(count) => {
                let size = *count as usize;
                self.advance_addr(size as Address);
            }
            Directive::Org(addr) => {
                self.set_addr(*addr as Address);
            }
            // These directives don't affect layout
            Directive::Globl(_)
            | Directive::Local(_)
            | Directive::Type(_, _)
            | Directive::Size(_, _)
            | Directive::File(_)
            | Directive::Ident(_)
            | Directive::Loc(_)
            | Directive::Bss => {
                // No address increment
            }
        }
        Ok(())
    }
}

impl Default for LayoutPass {
    fn default() -> Self {
        Self::new()
    }
}
