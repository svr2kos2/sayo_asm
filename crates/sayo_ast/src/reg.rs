use serde::{Deserialize, Serialize};
use std::fmt;

/// Register width in bits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegWidth {
    W8,
    W16,
    W24,
    W32,
}

impl RegWidth {
    pub fn bits(&self) -> u8 {
        match self {
            Self::W8 => 8,
            Self::W16 => 16,
            Self::W24 => 24,
            Self::W32 => 32,
        }
    }
}

/// Register access mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegAccess {
    R,   // Read-only
    W,   // Write-only
    RW,  // Read-Write
}

impl RegAccess {
    pub fn is_writable(&self) -> bool {
        matches!(self, Self::W | Self::RW)
    }
    
    pub fn is_readable(&self) -> bool {
        matches!(self, Self::R | Self::RW)
    }
}

/// Register definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterDef {
    pub index: u8,
    pub name: String,
    pub width: RegWidth,
    pub access: RegAccess,
    pub description: String,
}

/// Sayo register
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Register {
    // 8-bit parameter registers
    V0, V1, V2, V3,
    
    // 32-bit general purpose registers
    R0, R1, R2, R3, R4, R5, R6, R7,
    R8, R9, R10, R11, R12, R13, R14, R15,
    
    // Special registers
    DPTR,       // 32-bit, mapped to R4
    StarDPTR,   // *DPTR, 8-bit
    KeyIO,      // KEY_IO, 8-bit
    Zero,       // ZERO, 8-bit
    A,          // 32-bit, mapped to R6
    B,          // 32-bit, mapped to R7
    
    // Indirect addressing
    StarR0, StarR1, StarR2, StarR3,
    StarR4, StarR5, StarR6, StarR7,
    
    // Indirect addressing with width
    StarR0_16b, StarR1_16b, StarR2_16b, StarR3_16b,
    StarR4_16b, StarR5_16b, StarR6_16b, StarR7_16b,
    StarR0_32b, StarR1_32b, StarR2_32b, StarR3_32b,
    StarR4_32b, StarR5_32b, StarR6_32b, StarR7_32b,
    
    // System registers
    SysTimeMs,      // SYS_TIME_MS, 16-bit
    SysTimeS,       // SYS_TIME_S, 32-bit
    SysKbled,       // SYS_KBLED, 8-bit
    SysKeyCount,    // SYS_KEY_COUNT, 32-bit
    SysKeyLay,      // SYS_KEY_LAY, 8-bit
    ScriptAddr,     // SCRIPT_ADDR, 32-bit
    Random,         // RANDOM, 32-bit
    SysBleNum,      // SYS_BLE_NUM, 8-bit
    SysVolume,      // SYS_VOLUME, 8-bit
    SelectedLed,    // SELECTED_LED, 8-bit
    SelectedLedCol, // SELECTED_LED_COL, 24-bit
    AllLedCol,      // ALL_LED_COL, 24-bit
    CfgAddr,        // CFG_ADDR, 32-bit
    HeKeyLv,        // HE_KEY_LV, 32-bit
    SysUsbSusp,     // SYS_USB_SUSP, 8-bit
    
    // Global registers
    GlSize,         // GL_SIZE
    Gl(u8),         // GL_0 to GL_63
}

impl Register {
    /// Get the metadata (width and access mode) for this register
    pub fn metadata(&self) -> RegisterMetadata {
        match self {
            // 8-bit RW registers
            Self::V0 => RegisterMetadata::new(0x00, RegWidth::W8, RegAccess::RW),
            Self::V1 => RegisterMetadata::new(0x01, RegWidth::W8, RegAccess::RW),
            Self::V2 => RegisterMetadata::new(0x02, RegWidth::W8, RegAccess::RW),
            Self::V3 => RegisterMetadata::new(0x03, RegWidth::W8, RegAccess::RW),
            
            // 32-bit RW general purpose registers
            Self::R0 => RegisterMetadata::new(0x04, RegWidth::W32, RegAccess::RW),
            Self::R1 => RegisterMetadata::new(0x05, RegWidth::W32, RegAccess::RW),
            Self::R2 => RegisterMetadata::new(0x06, RegWidth::W32, RegAccess::RW),
            Self::R3 => RegisterMetadata::new(0x07, RegWidth::W32, RegAccess::RW),
            Self::R4 => RegisterMetadata::new(0x20, RegWidth::W32, RegAccess::RW),
            Self::R5 => RegisterMetadata::new(0x21, RegWidth::W32, RegAccess::RW),
            Self::R6 => RegisterMetadata::new(0x22, RegWidth::W32, RegAccess::RW),
            Self::R7 => RegisterMetadata::new(0x23, RegWidth::W32, RegAccess::RW),
            Self::R8 => RegisterMetadata::new(0x24, RegWidth::W32, RegAccess::RW),
            Self::R9 => RegisterMetadata::new(0x25, RegWidth::W32, RegAccess::RW),
            Self::R10 => RegisterMetadata::new(0x26, RegWidth::W32, RegAccess::RW),
            Self::R11 => RegisterMetadata::new(0x27, RegWidth::W32, RegAccess::RW),
            Self::R12 => RegisterMetadata::new(0x28, RegWidth::W32, RegAccess::RW),
            Self::R13 => RegisterMetadata::new(0x29, RegWidth::W32, RegAccess::RW),
            Self::R14 => RegisterMetadata::new(0x2A, RegWidth::W32, RegAccess::RW),
            Self::R15 => RegisterMetadata::new(0x2B, RegWidth::W32, RegAccess::RW),
            
            // Special registers
            Self::StarDPTR => RegisterMetadata::new(0x08, RegWidth::W8, RegAccess::R),  // Read-only
            Self::DPTR => RegisterMetadata::new(0x09, RegWidth::W32, RegAccess::RW),
            Self::KeyIO => RegisterMetadata::new(0x0A, RegWidth::W8, RegAccess::R),  // Read-only
            Self::Zero => RegisterMetadata::new(0x0F, RegWidth::W8, RegAccess::R),  // Read-only
            Self::A => RegisterMetadata::new(0x10, RegWidth::W32, RegAccess::RW),
            Self::B => RegisterMetadata::new(0x11, RegWidth::W32, RegAccess::RW),
            
            // Indirect addressing 8-bit
            Self::StarR0 => RegisterMetadata::new(0x0B, RegWidth::W8, RegAccess::RW),
            Self::StarR1 => RegisterMetadata::new(0x0C, RegWidth::W8, RegAccess::RW),
            Self::StarR2 => RegisterMetadata::new(0x0D, RegWidth::W8, RegAccess::RW),
            Self::StarR3 => RegisterMetadata::new(0x0E, RegWidth::W8, RegAccess::RW),
            Self::StarR4 => RegisterMetadata::new(0x2C, RegWidth::W8, RegAccess::RW),
            Self::StarR5 => RegisterMetadata::new(0x2D, RegWidth::W8, RegAccess::RW),
            Self::StarR6 => RegisterMetadata::new(0x2E, RegWidth::W8, RegAccess::RW),
            Self::StarR7 => RegisterMetadata::new(0x2F, RegWidth::W8, RegAccess::RW),
            
            // Indirect addressing 16-bit
            Self::StarR0_16b => RegisterMetadata::new(0x30, RegWidth::W16, RegAccess::RW),
            Self::StarR1_16b => RegisterMetadata::new(0x31, RegWidth::W16, RegAccess::RW),
            Self::StarR2_16b => RegisterMetadata::new(0x32, RegWidth::W16, RegAccess::RW),
            Self::StarR3_16b => RegisterMetadata::new(0x33, RegWidth::W16, RegAccess::RW),
            Self::StarR4_16b => RegisterMetadata::new(0x34, RegWidth::W16, RegAccess::RW),
            Self::StarR5_16b => RegisterMetadata::new(0x35, RegWidth::W16, RegAccess::RW),
            Self::StarR6_16b => RegisterMetadata::new(0x36, RegWidth::W16, RegAccess::RW),
            Self::StarR7_16b => RegisterMetadata::new(0x37, RegWidth::W16, RegAccess::RW),
            
            // Indirect addressing 32-bit
            Self::StarR0_32b => RegisterMetadata::new(0x38, RegWidth::W32, RegAccess::RW),
            Self::StarR1_32b => RegisterMetadata::new(0x39, RegWidth::W32, RegAccess::RW),
            Self::StarR2_32b => RegisterMetadata::new(0x3A, RegWidth::W32, RegAccess::RW),
            Self::StarR3_32b => RegisterMetadata::new(0x3B, RegWidth::W32, RegAccess::RW),
            Self::StarR4_32b => RegisterMetadata::new(0x3C, RegWidth::W32, RegAccess::RW),
            Self::StarR5_32b => RegisterMetadata::new(0x3D, RegWidth::W32, RegAccess::RW),
            Self::StarR6_32b => RegisterMetadata::new(0x3E, RegWidth::W32, RegAccess::RW),
            Self::StarR7_32b => RegisterMetadata::new(0x3F, RegWidth::W32, RegAccess::RW),
            
            // System registers
            Self::SysTimeMs => RegisterMetadata::new(0x12, RegWidth::W16, RegAccess::R),  // Read-only
            Self::SysTimeS => RegisterMetadata::new(0x13, RegWidth::W32, RegAccess::R),  // Read-only
            Self::SysKbled => RegisterMetadata::new(0x14, RegWidth::W8, RegAccess::RW),
            Self::SysKeyCount => RegisterMetadata::new(0x15, RegWidth::W32, RegAccess::R),  // Read-only
            Self::SysKeyLay => RegisterMetadata::new(0x16, RegWidth::W8, RegAccess::RW),
            Self::ScriptAddr => RegisterMetadata::new(0x17, RegWidth::W32, RegAccess::R),  // Read-only
            Self::Random => RegisterMetadata::new(0x18, RegWidth::W32, RegAccess::RW),
            Self::SysBleNum => RegisterMetadata::new(0x19, RegWidth::W8, RegAccess::RW),
            Self::SysVolume => RegisterMetadata::new(0x1A, RegWidth::W8, RegAccess::RW),
            Self::SelectedLed => RegisterMetadata::new(0x1B, RegWidth::W8, RegAccess::RW),
            Self::SelectedLedCol => RegisterMetadata::new(0x1C, RegWidth::W24, RegAccess::RW),
            Self::AllLedCol => RegisterMetadata::new(0x1D, RegWidth::W24, RegAccess::RW),
            Self::CfgAddr => RegisterMetadata::new(0x1E, RegWidth::W32, RegAccess::R),  // Read-only
            Self::HeKeyLv => RegisterMetadata::new(0x1F, RegWidth::W32, RegAccess::RW),
            Self::SysUsbSusp => RegisterMetadata::new(0x40, RegWidth::W8, RegAccess::RW),
            
            // Global registers
            Self::GlSize => RegisterMetadata::new(0x7F, RegWidth::W8, RegAccess::R),  // Read-only
            Self::Gl(n) => RegisterMetadata::new(0x80 + n, RegWidth::W32, RegAccess::RW),
        }
    }
    
    /// Get the human-readable description for this register
    pub fn description(&self) -> &'static str {
        match self {
            Self::V0 | Self::V1 | Self::V2 | Self::V3 => "Key parameters/General purpose register",
            Self::R0 | Self::R1 | Self::R2 | Self::R3 | Self::R4 | Self::R5 | Self::R6 | Self::R7 |
            Self::R8 | Self::R9 | Self::R10 | Self::R11 | Self::R12 | Self::R13 | Self::R14 | Self::R15 => "General purpose register",
            Self::DPTR => "Mapped to R4",
            Self::StarDPTR => "ROM addressing dedicated register, mapped to R4, shared address space",
            Self::KeyIO => "0=pressed",
            Self::StarR0 | Self::StarR1 | Self::StarR2 | Self::StarR3 => "RAM addressing using register (8-bit)",
            Self::StarR4 | Self::StarR5 | Self::StarR6 | Self::StarR7 => "RAM addressing using register (8-bit)",
            Self::StarR0_16b | Self::StarR1_16b | Self::StarR2_16b | Self::StarR3_16b |
            Self::StarR4_16b | Self::StarR5_16b | Self::StarR6_16b | Self::StarR7_16b => "RAM addressing using register (16-bit)",
            Self::StarR0_32b | Self::StarR1_32b | Self::StarR2_32b | Self::StarR3_32b |
            Self::StarR4_32b | Self::StarR5_32b | Self::StarR6_32b | Self::StarR7_32b => "RAM addressing using register (32-bit)",
            Self::Zero => "Always reads as 0",
            Self::A => "Dedicated register. Mapped to R6, shared address space. Can reduce code length for certain instructions",
            Self::B => "Dedicated register. Mapped to R7, shared address space. Can reduce code length for certain instructions",
            Self::SysTimeMs => "System time in milliseconds. Range 0-999",
            Self::SysTimeS => "System time in seconds",
            Self::SysKbled => "Keyboard LED status (Num Lock, Caps Lock, Scroll Lock, etc.)",
            Self::SysKeyCount => "Physical key press count",
            Self::SysKeyLay => "Keyboard layer. A keyboard may have multiple key layers",
            Self::ScriptAddr => "Script starting address",
            Self::Random => "R: Get random number W: Set random seed",
            Self::SysBleNum => "Bluetooth multi-device switching",
            Self::SysVolume => "Absolute system volume; currently ineffective on Windows",
            Self::SelectedLed => "Selected LED for operation. Default is the LED of the pressed key",
            Self::SelectedLedCol => "Modify the color of selected LED (RGB888)",
            Self::AllLedCol => "Modify the color of all LEDs (RGB888)",
            Self::CfgAddr => "Get current configuration file address",
            Self::HeKeyLv => "Magnetic axis key depth value in micrometers",
            Self::SysUsbSusp => "R: 1=USB in sleep state W: Wake up host",
            Self::GlSize => "Number of GL registers (minimum 4, maximum 64)",
            Self::Gl(_) => "General-purpose global register",
        }
    }
    
    /// Get common register variants for completion (without GL_0..GL_63)
    pub fn common_variants() -> Vec<(&'static str, Self)> {
        vec![
            ("V0", Self::V0),
            ("V1", Self::V1),
            ("V2", Self::V2),
            ("V3", Self::V3),
            ("R0", Self::R0),
            ("R1", Self::R1),
            ("R2", Self::R2),
            ("R3", Self::R3),
            ("R4", Self::R4),
            ("R5", Self::R5),
            ("R6", Self::R6),
            ("R7", Self::R7),
            ("R8", Self::R8),
            ("R9", Self::R9),
            ("R10", Self::R10),
            ("R11", Self::R11),
            ("R12", Self::R12),
            ("R13", Self::R13),
            ("R14", Self::R14),
            ("R15", Self::R15),
            ("DPTR", Self::DPTR),
            ("*DPTR", Self::StarDPTR),
            ("KEY_IO", Self::KeyIO),
            ("ZERO", Self::Zero),
            ("A", Self::A),
            ("B", Self::B),
            ("*R0", Self::StarR0),
            ("*R1", Self::StarR1),
            ("*R2", Self::StarR2),
            ("*R3", Self::StarR3),
            ("*R4", Self::StarR4),
            ("*R5", Self::StarR5),
            ("*R6", Self::StarR6),
            ("*R7", Self::StarR7),
            ("*R0_8b", Self::StarR0),
            ("*R1_8b", Self::StarR1),
            ("*R2_8b", Self::StarR2),
            ("*R3_8b", Self::StarR3),
            ("*R4_8b", Self::StarR4),
            ("*R5_8b", Self::StarR5),
            ("*R6_8b", Self::StarR6),
            ("*R7_8b", Self::StarR7),
            ("*R0_16b", Self::StarR0_16b),
            ("*R1_16b", Self::StarR1_16b),
            ("*R2_16b", Self::StarR2_16b),
            ("*R3_16b", Self::StarR3_16b),
            ("*R4_16b", Self::StarR4_16b),
            ("*R5_16b", Self::StarR5_16b),
            ("*R6_16b", Self::StarR6_16b),
            ("*R7_16b", Self::StarR7_16b),
            ("*R0_32b", Self::StarR0_32b),
            ("*R1_32b", Self::StarR1_32b),
            ("*R2_32b", Self::StarR2_32b),
            ("*R3_32b", Self::StarR3_32b),
            ("*R4_32b", Self::StarR4_32b),
            ("*R5_32b", Self::StarR5_32b),
            ("*R6_32b", Self::StarR6_32b),
            ("*R7_32b", Self::StarR7_32b),
            ("SYS_TIME_MS", Self::SysTimeMs),
            ("SYS_TIME_S", Self::SysTimeS),
            ("SYS_KBLED", Self::SysKbled),
            ("SYS_KEY_COUNT", Self::SysKeyCount),
            ("SYS_KEY_LAY", Self::SysKeyLay),
            ("SCRIPT_ADDR", Self::ScriptAddr),
            ("RANDOM", Self::Random),
            ("SYS_BLE_NUM", Self::SysBleNum),
            ("SYS_VOLUME", Self::SysVolume),
            ("SELECTED_LED", Self::SelectedLed),
            ("SELECTED_LED_COL", Self::SelectedLedCol),
            ("ALL_LED_COL", Self::AllLedCol),
            ("CFG_ADDR", Self::CfgAddr),
            ("HE_KEY_LV", Self::HeKeyLv),
            ("SYS_USB_SUSP", Self::SysUsbSusp),
            ("GL_SIZE", Self::GlSize),
        ]
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "V0" => Some(Self::V0),
            "V1" => Some(Self::V1),
            "V2" => Some(Self::V2),
            "V3" => Some(Self::V3),
            "R0" => Some(Self::R0),
            "R1" => Some(Self::R1),
            "R2" => Some(Self::R2),
            "R3" => Some(Self::R3),
            "R4" => Some(Self::R4),
            "R5" => Some(Self::R5),
            "R6" => Some(Self::R6),
            "R7" => Some(Self::R7),
            "R8" => Some(Self::R8),
            "R9" => Some(Self::R9),
            "R10" => Some(Self::R10),
            "R11" => Some(Self::R11),
            "R12" => Some(Self::R12),
            "R13" => Some(Self::R13),
            "R14" => Some(Self::R14),
            "R15" => Some(Self::R15),
            "DPTR" => Some(Self::DPTR),
            "*DPTR" => Some(Self::StarDPTR),
            "KEY_IO" => Some(Self::KeyIO),
            "ZERO" => Some(Self::Zero),
            "A" => Some(Self::A),
            "B" => Some(Self::B),
            "*R0" => Some(Self::StarR0),
            "*R1" => Some(Self::StarR1),
            "*R2" => Some(Self::StarR2),
            "*R3" => Some(Self::StarR3),
            "*R4" => Some(Self::StarR4),
            "*R5" => Some(Self::StarR5),
            "*R6" => Some(Self::StarR6),
            "*R7" => Some(Self::StarR7),
            // Indirect with 8-bit width specifier (maps to default StarRx)
            "*R0_8B" => Some(Self::StarR0),
            "*R1_8B" => Some(Self::StarR1),
            "*R2_8B" => Some(Self::StarR2),
            "*R3_8B" => Some(Self::StarR3),
            "*R4_8B" => Some(Self::StarR4),
            "*R5_8B" => Some(Self::StarR5),
            "*R6_8B" => Some(Self::StarR6),
            "*R7_8B" => Some(Self::StarR7),
            // Indirect with width specifiers
            "*R0_16B" => Some(Self::StarR0_16b),
            "*R1_16B" => Some(Self::StarR1_16b),
            "*R2_16B" => Some(Self::StarR2_16b),
            "*R3_16B" => Some(Self::StarR3_16b),
            "*R4_16B" => Some(Self::StarR4_16b),
            "*R5_16B" => Some(Self::StarR5_16b),
            "*R6_16B" => Some(Self::StarR6_16b),
            "*R7_16B" => Some(Self::StarR7_16b),
            "*R0_32B" => Some(Self::StarR0_32b),
            "*R1_32B" => Some(Self::StarR1_32b),
            "*R2_32B" => Some(Self::StarR2_32b),
            "*R3_32B" => Some(Self::StarR3_32b),
            "*R4_32B" => Some(Self::StarR4_32b),
            "*R5_32B" => Some(Self::StarR5_32b),
            "*R6_32B" => Some(Self::StarR6_32b),
            "*R7_32B" => Some(Self::StarR7_32b),
            "SYS_TIME_MS" => Some(Self::SysTimeMs),
            "SYS_TIME_S" => Some(Self::SysTimeS),
            "SYS_KBLED" => Some(Self::SysKbled),
            "SYS_KEY_COUNT" => Some(Self::SysKeyCount),
            "SYS_KEY_LAY" => Some(Self::SysKeyLay),
            "SCRIPT_ADDR" => Some(Self::ScriptAddr),
            "RANDOM" => Some(Self::Random),
            "SYS_BLE_NUM" => Some(Self::SysBleNum),
            "SYS_VOLUME" => Some(Self::SysVolume),
            "SELECTED_LED" => Some(Self::SelectedLed),
            "SELECTED_LED_COL" => Some(Self::SelectedLedCol),
            "ALL_LED_COL" => Some(Self::AllLedCol),
            "CFG_ADDR" => Some(Self::CfgAddr),
            "HE_KEY_LV" => Some(Self::HeKeyLv),
            "SYS_USB_SUSP" => Some(Self::SysUsbSusp),
            "GL_SIZE" => Some(Self::GlSize),
            _ => {
                // Try to parse GL_n
                if s.starts_with("GL_") {
                    let num_str = &s[3..];
                    if let Ok(num) = num_str.parse::<u8>() {
                        if num < 64 {
                            return Some(Self::Gl(num));
                        }
                    }
                }
                None
            }
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V0 => write!(f, "V0"),
            Self::V1 => write!(f, "V1"),
            Self::V2 => write!(f, "V2"),
            Self::V3 => write!(f, "V3"),
            Self::R0 => write!(f, "R0"),
            Self::R1 => write!(f, "R1"),
            Self::R2 => write!(f, "R2"),
            Self::R3 => write!(f, "R3"),
            Self::R4 => write!(f, "R4"),
            Self::R5 => write!(f, "R5"),
            Self::R6 => write!(f, "R6"),
            Self::R7 => write!(f, "R7"),
            Self::R8 => write!(f, "R8"),
            Self::R9 => write!(f, "R9"),
            Self::R10 => write!(f, "R10"),
            Self::R11 => write!(f, "R11"),
            Self::R12 => write!(f, "R12"),
            Self::R13 => write!(f, "R13"),
            Self::R14 => write!(f, "R14"),
            Self::R15 => write!(f, "R15"),
            Self::DPTR => write!(f, "DPTR"),
            Self::StarDPTR => write!(f, "*DPTR"),
            Self::KeyIO => write!(f, "KEY_IO"),
            Self::Zero => write!(f, "ZERO"),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::StarR0 => write!(f, "*R0"),
            Self::StarR1 => write!(f, "*R1"),
            Self::StarR2 => write!(f, "*R2"),
            Self::StarR3 => write!(f, "*R3"),
            Self::StarR4 => write!(f, "*R4"),
            Self::StarR5 => write!(f, "*R5"),
            Self::StarR6 => write!(f, "*R6"),
            Self::StarR7 => write!(f, "*R7"),
            Self::SysTimeMs => write!(f, "SYS_TIME_MS"),
            Self::SysTimeS => write!(f, "SYS_TIME_S"),
            Self::SysKbled => write!(f, "SYS_KBLED"),
            Self::SysKeyCount => write!(f, "SYS_KEY_COUNT"),
            Self::SysKeyLay => write!(f, "SYS_KEY_LAY"),
            Self::ScriptAddr => write!(f, "SCRIPT_ADDR"),
            Self::Random => write!(f, "RANDOM"),
            Self::SysBleNum => write!(f, "SYS_BLE_NUM"),
            Self::SysVolume => write!(f, "SYS_VOLUME"),
            Self::SelectedLed => write!(f, "SELECTED_LED"),
            Self::SelectedLedCol => write!(f, "SELECTED_LED_COL"),
            Self::AllLedCol => write!(f, "ALL_LED_COL"),
            Self::CfgAddr => write!(f, "CFG_ADDR"),
            Self::HeKeyLv => write!(f, "HE_KEY_LV"),
            Self::SysUsbSusp => write!(f, "SYS_USB_SUSP"),
            Self::GlSize => write!(f, "GL_SIZE"),
            Self::Gl(n) => write!(f, "GL_{}", n),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Register metadata containing access and width information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterMetadata {
    pub index: u8,
    pub width: RegWidth,
    pub access: RegAccess,
}

impl RegisterMetadata {
    pub fn new(index: u8, width: RegWidth, access: RegAccess) -> Self {
        Self { index, width, access }
    }
    
    pub fn is_read_only(&self) -> bool {
        matches!(self.access, RegAccess::R)
    }
    
    pub fn is_writable(&self) -> bool {
        self.access.is_writable()
    }
}

