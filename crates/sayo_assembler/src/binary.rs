//! Binary format constants and utilities for Sayo assembler output.
//!
//! Binary Layout:
//! ```text
//! Offset  Size  Description
//! ------  ----  -----------
//! 0x0000  3     CALL main instruction
//! 0x0003  1     EXIT instruction
//! 0x0004  4     Magic "SAYO"
//! 0x0008  1     Version byte
//! 0x0009  2     Text section size (u16 LE)
//! 0x000B  1     Reserved (0x00)
//! 0x000C  N     Text section (code)
//! 0x000C+N M    Data section (linear layout, no length prefixes)
//! ```
//!
//! The data section is laid out linearly - data labels point directly to the
//! memory location where the data bytes are stored. This allows code to directly
//! dereference data addresses without any overhead.

/// Magic bytes: "SAYO"
pub const MAGIC: [u8; 4] = [0x53, 0x41, 0x59, 0x4F];

/// Current format version
pub const VERSION: u8 = 0x01;

/// CALL opcode
pub const CALL_OPCODE: u8 = 0x54;

/// EXIT opcode
pub const EXIT_OPCODE: u8 = 0xFF;

/// Header size in bytes (bootstrap instructions + header fields)
/// - CALL main: 3 bytes (opcode + u16 addr)
/// - EXIT: 1 byte (opcode)
/// - Magic: 4 bytes
/// - Version: 1 byte
/// - Text size: 2 bytes
/// - Reserved: 1 byte
/// Total: 12 bytes
pub const HEADER_SIZE: u32 = 12;

/// Offset of the CALL instruction (always 0)
pub const CALL_OFFSET: u32 = 0;

/// Offset of the EXIT instruction
pub const EXIT_OFFSET: u32 = 3;

/// Offset of the magic bytes
pub const MAGIC_OFFSET: u32 = 4;

/// Offset of the version byte
pub const VERSION_OFFSET: u32 = 8;

/// Offset of the text size field
pub const TEXT_SIZE_OFFSET: u32 = 9;

/// Offset of the reserved byte
pub const RESERVED_OFFSET: u32 = 11;

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
    
    // CALL main instruction
    header.push(CALL_OPCODE);
    header.extend_from_slice(&main_addr.to_be_bytes());
    
    // EXIT instruction
    header.push(EXIT_OPCODE);
    
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
        
        // CALL instruction
        assert_eq!(header[0], CALL_OPCODE);
        assert_eq!(header[1], 0x00); // High byte of address (big-endian)
        assert_eq!(header[2], 0x20); // Low byte of address
        
        // EXIT instruction
        assert_eq!(header[3], EXIT_OPCODE);
        
        // Magic
        assert_eq!(&header[4..8], b"SAYO");
        
        // Version
        assert_eq!(header[8], VERSION);
        
        // Text size
        assert_eq!(header[9], 0x00); // Low byte
        assert_eq!(header[10], 0x01); // High byte
        
        // Reserved
        assert_eq!(header[11], 0x00);
    }
}
