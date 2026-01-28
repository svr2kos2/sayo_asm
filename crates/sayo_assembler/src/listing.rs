use crate::layout::Layout;
use sayo_ast::{Item, Program};
use std::collections::HashMap;
use std::error::Error;

/// Listing generator - creates show-encoding format output
/// 
/// Output format per line:
/// - For instructions after label or address change: "MOV8 R2, 2                                        @ 0x0010 -> [0x6f,0x60,0x40]"
/// - For labels: "main:                                                 @ 0x0010"
/// - Other lines: just the source text without annotation
/// - Left side is padded to 60 chars; if source is > 60 chars, annotation is appended directly
pub struct Listing;

/// Information about a source line for listing purposes
struct LineInfo {
    address: u32,
    encoding: Option<Vec<u8>>,
    is_label: bool,
}

impl Listing {
    /// Generate a listing file with address and encoding annotations
    /// 
    /// This implementation shows address/encoding only after labels or when address changes.
    pub fn generate(
        source: &str,
        program: &Program,
        layout: &Layout,
        machine_code: &[u8],
    ) -> Result<String, Box<dyn Error>> {
        // Build line info using original source (since spans are based on original bytes)
        let line_info = Self::build_line_info(source, program, layout, machine_code);
        
        // Replace tabs with 4 spaces for consistent alignment in output
        let source_normalized = source.replace('\t', "    ");
        
        // Generate output line by line
        let mut output = String::new();
        
        // Add header information
        Self::generate_header(&mut output, machine_code, layout)?;
        
        let mut last_end_addr: Option<u32> = None;
        let mut show_after_label = false;
        
        for (line_num, line_text) in source_normalized.lines().enumerate() {
            let line_num_1based = line_num + 1;
            
            if let Some(info) = line_info.get(&line_num_1based) {
                // Check if this is a label
                if info.is_label {
                    show_after_label = true;
                }
                
                // Determine if we should show address/encoding
                // Show if: it's a label, or has encoding (instructions/data change address), or address jumped
                let addr_jumped = last_end_addr.map_or(true, |last| info.address != last);
                let should_show = info.is_label || info.encoding.is_some() || show_after_label || addr_jumped;
                
                if should_show {
                    // Format with annotation
                    let trimmed = line_text.trim_end();
                    let annotation = if let Some(ref bytes) = info.encoding {
                        // With encoding
                        let encoding_str = Self::format_bytes(bytes);
                        format!("; @ 0x{:04x} -> [{}]", info.address, encoding_str)
                    } else {
                        // Address only
                        format!("; @ 0x{:04x}", info.address)
                    };
                    
                    let formatted = if trimmed.len() >= 64 {
                        // If source is 64+ chars, append directly
                        format!("{}  {}\n", trimmed, annotation)
                    } else {
                        // Pad to 64 chars
                        format!("{:<64} {}\n", trimmed, annotation)
                    };
                    output.push_str(&formatted);
                    
                    // Update tracking
                    let inst_len = info.encoding.as_ref().map_or(0, |b| b.len()) as u32;
                    last_end_addr = Some(info.address + inst_len);
                    if !info.is_label {
                        show_after_label = false;
                    }
                } else {
                    // No annotation needed
                    output.push_str(line_text.trim_end());
                    output.push('\n');
                    let inst_len = info.encoding.as_ref().map_or(0, |b| b.len()) as u32;
                    last_end_addr = Some(info.address + inst_len);
                }
            } else {
                // Line not in program (comments, empty lines, etc.)
                output.push_str(line_text.trim_end());
                output.push('\n');
            }
        }

        Ok(output)
    }

    /// Generate header information for the listing file
    fn generate_header(output: &mut String, machine_code: &[u8], layout: &Layout) -> Result<(), Box<dyn Error>> {
        // Header structure:
        // 0x0000: CALL main (3 bytes)
        // 0x0003: EXIT (1 byte)
        // 0x0004: Magic "SAYO" (4 bytes)
        // 0x0008: Version (1 byte)
        // 0x0009: Text size (2 bytes, little-endian)
        // 0x000B: Reserved (1 byte)
        // 0x000C: Assembly begins
        
        const HEADER_SIZE: usize = 12;
        
        if machine_code.len() < HEADER_SIZE {
            return Err("Machine code too short for header".into());
        }
        
        // CALL instruction (0x0000-0x0002)
        let call_bytes = &machine_code[0..3];
        output.push_str(&format!(
            "    ; CALL main                                                  ; @ 0x0000 -> [{}]\n",
            Self::format_bytes(call_bytes)
        ));
        
        // EXIT instruction (0x0003)
        let exit_byte = machine_code[3];
        output.push_str(&format!(
            "    ; EXIT                                                       ; @ 0x0003 -> [0x{:02x}]\n",
            exit_byte
        ));
        
        // Header marker
        output.push_str("; header:                                                        ; @ 0x0004\n");
        
        // Magic "SAYO" (0x0004-0x0007)
        let magic_bytes = &machine_code[4..8];
        output.push_str(&format!(
            "    ; \"SAYO\"                                                     ; @ 0x0004 -> [{}]\n",
            Self::format_bytes(magic_bytes)
        ));
        
        // Version (0x0008)
        let version = machine_code[8];
        output.push_str(&format!(
            "    ; version {}                                                  ; @ 0x0008 -> [0x{:02x}]\n",
            version, version
        ));
        
        // Text size (0x0009-0x000A)
        let text_size = u16::from_le_bytes([machine_code[9], machine_code[10]]);
        let text_size_padding = " ".repeat(8 - text_size.to_string().len());
        
        output.push_str(&format!(
            "    ; text_size {}      {text_size_padding}                                   ; @ 0x0009 -> [0x{:02x}, 0x{:02x}]\n",
            text_size, machine_code[9], machine_code[10]
        ));
        
        // Reserved byte (0x000B)
        output.push_str(&format!(
            "    ; reserved byte                                              ; @ 0x000b -> [0x{:02x}]\n",
            machine_code[11]
        ));
        
        // Assembly begin marker
        output.push_str(";assembly begin                                                  ; @ 0x000c\n");
        
        Ok(())
    }

    /// Build a map from 1-based line numbers to line info
    fn build_line_info(
        source: &str,
        program: &Program,
        layout: &Layout,
        machine_code: &[u8],
    ) -> HashMap<usize, LineInfo> {
        let mut line_info: HashMap<usize, LineInfo> = HashMap::new();
        let line_starts = Self::compute_line_starts(source);
        
        // Track position in machine code - start after header
        // The machine_code includes: header (11 bytes) + text section + data section
        // We use item addresses directly to index into machine_code

        for (idx, item) in program.items.iter().enumerate() {
            let item_addr = layout.item_addresses[idx];
            let span = &item.span;
            
            // Find line number for this item's start position
            let line_num = Self::offset_to_line(&line_starts, span.start);
            
            // Skip if we already have info for this line (keep first item's info)
            if line_info.contains_key(&line_num) {
                continue;
            }

            match &item.node {
                Item::Instruction(instr) => {
                    let metadata = instr.mnemonic.metadata();
                    let inst_len = metadata.length as usize;

                    // Use item_addr directly as offset into machine_code
                    // Since machine_code includes header, and item_addr starts at HEADER_SIZE
                    let code_offset = item_addr as usize;
                    let inst_bytes = if code_offset + inst_len <= machine_code.len() {
                        machine_code[code_offset..code_offset + inst_len].to_vec()
                    } else {
                        vec![]
                    };

                    line_info.insert(line_num, LineInfo {
                        address: item_addr,
                        encoding: Some(inst_bytes),
                        is_label: false,
                    });
                }
                Item::Label(_) => {
                    line_info.insert(line_num, LineInfo {
                        address: item_addr,
                        encoding: None,
                        is_label: true,
                    });
                }
                Item::Directive(dir) => {
                    use sayo_ast::Directive;
                    let bytes_emitted = match dir {
                        Directive::Byte(values) => values.len(),
                        Directive::Word(values) | Directive::Short(values) => values.len() * 2,
                        Directive::Long(values) => values.len() * 4,
                        Directive::Quad(values) => values.len() * 8,
                        Directive::Ascii(s) => s.len(),
                        Directive::Asciz(s) => s.len() + 1, // +1 for null terminator
                        Directive::Zero(count) | Directive::Skip(count) => *count as usize,
                        _ => 0,
                    };

                    if bytes_emitted > 0 {
                        // For data directives in text section, use item_addr
                        // For data section, the encoding is in the structured data format
                        // which has length prefixes, so we can't easily extract it
                        let code_offset = item_addr as usize;
                        let dir_bytes = if code_offset + bytes_emitted <= machine_code.len() {
                            machine_code[code_offset..code_offset + bytes_emitted].to_vec()
                        } else {
                            vec![]
                        };

                        line_info.insert(line_num, LineInfo {
                            address: item_addr,
                            encoding: if dir_bytes.is_empty() { None } else { Some(dir_bytes) },
                            is_label: false,
                        });
                    } else {
                        line_info.insert(line_num, LineInfo {
                            address: item_addr,
                            encoding: None,
                            is_label: false,
                        });
                    }
                }
            }
        }

        line_info
    }

    /// Compute byte offset of each line start
    fn compute_line_starts(source: &str) -> Vec<usize> {
        let mut starts = vec![0];
        for (i, c) in source.char_indices() {
            if c == '\n' {
                starts.push(i + 1);
            }
        }
        starts
    }

    /// Convert byte offset to 1-based line number
    fn offset_to_line(line_starts: &[usize], offset: usize) -> usize {
        match line_starts.binary_search(&offset) {
            Ok(idx) => idx + 1,
            Err(idx) => idx, // idx is where it would be inserted, so line is idx (0-indexed) -> +0 for 1-based since we want previous line
        }
    }

    /// Format byte array as hex string "0x6f,0x60,0x40"
    fn format_bytes(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("0x{:02x}", b))
            .collect::<Vec<_>>()
            .join(",")
    }
}
