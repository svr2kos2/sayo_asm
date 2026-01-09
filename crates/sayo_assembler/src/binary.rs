//! Binary format constants and utilities for Sayo assembler output.
//!
//! Binary Layout:
//! ```text
//! Offset  Size  Description
//! ------  ----  -----------
//! 0x0000  3     JMP main instruction
//! 0x0003  4     Magic "SAYO"
//! 0x0007  1     Version byte
//! 0x0008  2     Text section size (u16 LE)
//! 0x000A  1     Reserved (0x00)
//! 0x000B  N     Text section (code)
//! 0x000B+N M    Data section (linear layout, no length prefixes)
//! ```
//!
//! The data section is laid out linearly - data labels point directly to the
//! memory location where the data bytes are stored. This allows code to directly
//! dereference data addresses without any overhead.

/// Magic bytes: "SAYO"
pub const MAGIC: [u8; 4] = [0x53, 0x41, 0x59, 0x4F];

/// Current format version
pub const VERSION: u8 = 0x01;

/// JMP opcode
pub const JMP_OPCODE: u8 = 0x02;

/// Header size in bytes (JMP instruction + header fields)
/// - JMP main: 3 bytes (opcode + u16 addr)
/// - Magic: 4 bytes
/// - Version: 1 byte
/// - Text size: 2 bytes
/// - Reserved: 1 byte
/// Total: 11 bytes
pub const HEADER_SIZE: u32 = 11;

/// Offset of the JMP instruction (always 0)
pub const JMP_OFFSET: u32 = 0;

/// Offset of the magic bytes
pub const MAGIC_OFFSET: u32 = 3;

/// Offset of the version byte
pub const VERSION_OFFSET: u32 = 7;

/// Offset of the text size field
pub const TEXT_SIZE_OFFSET: u32 = 8;

/// Offset of the reserved byte
pub const RESERVED_OFFSET: u32 = 10;

/// Error types for binary generation
#[derive(Debug, thiserror::Error)]
pub enum BinaryError {
    #[error("No 'main' label found. Entry point is required.")]
    NoMainLabel,

    #[error("Text section size {0} exceeds maximum (65535 bytes)")]
    TextSectionTooLarge(u32),
}

/// Generate the binary header
pub fn generate_header(main_addr: u16, text_size: u16) -> Vec<u8> {
    let mut header = Vec::with_capacity(HEADER_SIZE as usize);
    
    // JMP main instruction
    header.push(JMP_OPCODE);
    header.extend_from_slice(&main_addr.to_be_bytes());
    
    // Magic "SAYO"
    header.extend_from_slice(&MAGIC);
    
    // Version
    header.push(VERSION);
    
    // Text section size
    header.extend_from_slice(&text_size.to_le_bytes());
    
    // Reserved byte
    header.push(0x00);
    
    header
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_size() {
        let header = generate_header(0x000B, 100);
        assert_eq!(header.len(), HEADER_SIZE as usize);
    }

    #[test]
    fn test_header_content() {
        let header = generate_header(0x0020, 0x0100);
        
        // JMP instruction
        assert_eq!(header[0], JMP_OPCODE);
        assert_eq!(header[1], 0x20); // Low byte of address
        assert_eq!(header[2], 0x00); // High byte of address
        
        // Magic
        assert_eq!(&header[3..7], b"SAYO");
        
        // Version
        assert_eq!(header[7], VERSION);
        
        // Text size
        assert_eq!(header[8], 0x00); // Low byte
        assert_eq!(header[9], 0x01); // High byte
        
        // Reserved
        assert_eq!(header[10], 0x00);
    }
}
