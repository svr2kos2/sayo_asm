pub mod ast;
pub mod instr;
pub mod reg;
pub mod span;

pub use ast::*;
pub use instr::*;
pub use reg::*;
pub use span::*;

/// Generate markdown documentation for all instructions
pub fn generate_instruction_markdown() -> String {
    let mut md = String::new();
    
    md.push_str("# Sayo Assembly Instruction Reference\n\n");
    md.push_str("Auto-generated documentation containing all Sayo assembly instructions.\n\n");
    md.push_str("## Instruction List\n\n");
    md.push_str("| Mnemonic | Opcode(Hex) | Length | Operands | Description | Notes |\n");
    md.push_str("|--------|-------------|----------|--------|------|------|\n");
    
    let mnemonics = Mnemonic::all_variants();
    let mut sorted_mnemonics: Vec<_> = mnemonics.iter()
        .map(|m| (m, m.metadata()))
        .collect();
    
    // Sort by opcode
    sorted_mnemonics.sort_by_key(|(_, meta)| meta.opcode);
    
    for (mnemonic, meta) in sorted_mnemonics {
        let opcode_hex = format!("0x{:02X}", meta.opcode);
        let operands = format_operands(&meta.operands);
        let (description, note) = mnemonic.description();
        
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            mnemonic,
            opcode_hex,
            meta.length,
            operands,
            description,
            note
        ));
    }
    
    // Add categorized sections
    md.push_str("\n## By Category\n\n");
    
    // Control Flow
    md.push_str("### Control Flow Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::END, Mnemonic::NOP, Mnemonic::JMP, Mnemonic::SJMP, Mnemonic::AJMP,
        Mnemonic::CALL, Mnemonic::RET, Mnemonic::JZ, Mnemonic::JNZ, Mnemonic::JC, Mnemonic::JNC,
        Mnemonic::DJNZ, Mnemonic::JA, Mnemonic::JB, Mnemonic::JG, Mnemonic::JL,
        Mnemonic::CJNE, Mnemonic::RES, Mnemonic::EXIT,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    // Sleep/Delay
    md.push_str("\n### Delay Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::SLEEP, Mnemonic::SLEEP_X256, Mnemonic::SLEEP_U16,
        Mnemonic::SLEEP_RAND, Mnemonic::SLEEP_RAND_X256, Mnemonic::SLEEP_RAND_U16,
        Mnemonic::SLEEP_VAL, Mnemonic::SLEEP_X256_VAL, 
        Mnemonic::SLEEP_RAND_VAL, Mnemonic::SLEEP_RAND_X8_VAL,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    // Arithmetic
    md.push_str("\n### Arithmetic Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::ADD, Mnemonic::ADD8, Mnemonic::ADD16, Mnemonic::ADD32,
        Mnemonic::SUB, Mnemonic::SUB8, Mnemonic::SUB16, Mnemonic::SUB32,
        Mnemonic::MUL_A, Mnemonic::DIV_A, Mnemonic::IMUL_A,
        Mnemonic::ADD_R, Mnemonic::SUB_R, Mnemonic::MUL_R, Mnemonic::DIV_R, Mnemonic::MOD_R, Mnemonic::IMUL_R,
        Mnemonic::INC, Mnemonic::DEC,
        Mnemonic::ADD_A, Mnemonic::ADD8_A, Mnemonic::SUB_A, Mnemonic::SUB8_A,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    // Logical
    md.push_str("\n### Logical Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::AND, Mnemonic::AND8, Mnemonic::AND16, Mnemonic::AND32,
        Mnemonic::OR, Mnemonic::OR8, Mnemonic::OR16, Mnemonic::OR32,
        Mnemonic::XOR, Mnemonic::XOR8, Mnemonic::XOR16, Mnemonic::XOR32,
        Mnemonic::NOT, Mnemonic::SHL, Mnemonic::SHL8, Mnemonic::SHR, Mnemonic::SHR8,
        Mnemonic::AND_R, Mnemonic::OR_R, Mnemonic::XOR_R,
        Mnemonic::OR_A, Mnemonic::OR8_A,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    // Data Transfer
    md.push_str("\n### Data Transfer Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::MOV, Mnemonic::MOV8, Mnemonic::MOV16, Mnemonic::MOV32,
        Mnemonic::MOVSX8b, Mnemonic::MOVSX16b, Mnemonic::MOV8SX, Mnemonic::MOV16SX,
        Mnemonic::XCH, Mnemonic::PUSH, Mnemonic::POP,
        Mnemonic::CLR, Mnemonic::CMP,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    // HID - Keyboard
    md.push_str("\n### HID Keyboard Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::PRESS_SK, Mnemonic::PRESS_GK, Mnemonic::RELEASE_SK, Mnemonic::RELEASE_GK,
        Mnemonic::PRESS_SK_VAL, Mnemonic::PRESS_GK_VAL, 
        Mnemonic::RELEASE_SK_VAL, Mnemonic::RELEASE_GK_VAL,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    // HID - Mouse
    md.push_str("\n### HID Mouse Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::PRESS_MK, Mnemonic::RELEASE_MK,
        Mnemonic::PRESS_MK_VAL, Mnemonic::RELEASE_MK_VAL,
        Mnemonic::MO_XYZ, Mnemonic::MO_XYZ_VAL,
        Mnemonic::TB_XY, Mnemonic::TB_XY_VAL,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    // HID - Media & Gamepad
    md.push_str("\n### HID Media & Gamepad Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::PRESS_MU, Mnemonic::RELEASE_MU,
        Mnemonic::PRESS_MU_VAL, Mnemonic::RELEASE_MU_VAL,
        Mnemonic::PRESS_GAK, Mnemonic::RELEASE_GAK,
        Mnemonic::PRESS_GAK_VAL, Mnemonic::RELEASE_GAK_VAL,
        Mnemonic::GA_XYZ, Mnemonic::GA_XYZ_VAL,
        Mnemonic::DIAL_DATA, Mnemonic::DIAL_DATA_VAL,
        Mnemonic::UPDATE,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    // System & LED
    md.push_str("\n### System & LED Control Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::LED_CTRL, Mnemonic::LED_COL,
        Mnemonic::START, Mnemonic::STOP, Mnemonic::SYCON,
        Mnemonic::JMP_TO_SCRIPT,
        Mnemonic::MODE_JOG, Mnemonic::WAIT_IF_RELEASE, Mnemonic::WAIT_IF_PRESS,
        Mnemonic::EXIT_IF_RELEAS, Mnemonic::EXIT_IF_PRESS, Mnemonic::EXIT_IF_ANYKEY,
        Mnemonic::WHILE_UPDATE,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    // Memory & Threading
    md.push_str("\n### Memory & Threading Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::MALLOC, Mnemonic::FREE,
        Mnemonic::NEW_THREAD,
        Mnemonic::MOV_PC2REG, Mnemonic::VALUE_RELOAD,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    // Print/Debug
    md.push_str("\n### Print & Debug Instructions\n\n");
    md.push_str("| Instruction | Description |\n");
    md.push_str("|-------------|-------------|\n");
    for mnemonic in &[
        Mnemonic::PRINT_REG, Mnemonic::C2K, Mnemonic::U2K, 
        Mnemonic::C2K_RAND, Mnemonic::U2K_REG,
    ] {
        let (desc, note) = mnemonic.description();
        let full_desc = if note.is_empty() { desc.to_string() } else { format!("{} ({})", desc, note) };
        md.push_str(&format!("| {} | {} |\n", mnemonic, full_desc));
    }
    
    md
}

/// Generate markdown documentation for all registers
pub fn generate_register_markdown() -> String {
    let mut md = String::new();
    
    md.push_str("# Sayo Assembly Register Reference\n\n");
    md.push_str("Auto-generated documentation containing all Sayo assembly registers.\n\n");
    md.push_str("## Register List\n\n");
    md.push_str("| Register | Index(Hex) | Width | Access | Description |\n");
    md.push_str("|--------|-----------|------|------|------|\n");
    
    let registers = Register::common_variants();
    for (name, reg) in registers {
        let meta = reg.metadata();
        let index_hex = format!("0x{:02X}", meta.index);
        let width = format!("{}-bit", meta.width.bits());
        let access = format!("{:?}", meta.access);
        let description = reg.description();
        
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            name,
            index_hex,
            width,
            access,
            description
        ));
    }
    
    // Global registers
    md.push_str("\n### Global Registers (GL_0 to GL_63)\n\n");
    md.push_str("Global registers GL_0 to GL_63 are 32-bit read-write registers for global data storage.\n");
    md.push_str("The number of available registers can be read from GL_SIZE register (minimum 4, maximum 64).\n\n");
    md.push_str("| Register | Index(Hex) | Width | Access |\n");
    md.push_str("|--------|-----------|------|------|\n");
    for i in 0..64 {
        let reg = Register::Gl(i);
        let meta = reg.metadata();
        let index_hex = format!("0x{:02X}", meta.index);
        md.push_str(&format!("| GL_{} | {} | 32-bit | RW |\n", i, index_hex));
    }
    
    // Categorized sections
    md.push_str("\n## By Category\n\n");
    
    // General Purpose
    md.push_str("### General Purpose Registers\n\n");
    md.push_str("| Register | Description |\n");
    md.push_str("|----------|-------------|\\n");
    md.push_str("| V0-V3 | 8-bit parameter/general purpose registers |\n");
    md.push_str("| R0-R15 | 32-bit general purpose registers |\n");
    md.push_str("| A | 32-bit dedicated register (mapped to R6), can reduce code size for certain instructions |\n");
    md.push_str("| B | 32-bit dedicated register (mapped to R7), can reduce code size for certain instructions |\n");
    
    // Indirect Addressing
    md.push_str("\n### Indirect Addressing Registers\n\n");
    md.push_str("| Register | Description |\n");
    md.push_str("|----------|-------------|\\n");
    md.push_str("| *R0-*R7 | 8-bit indirect addressing (using R0-R7 as address) |\n");
    md.push_str("| *R0_16b-*R7_16b | 16-bit indirect addressing |\n");
    md.push_str("| *R0_32b-*R7_32b | 32-bit indirect addressing |\n");
    md.push_str("| DPTR | 32-bit data pointer (mapped to R4) |\n");
    md.push_str("| *DPTR | 8-bit ROM addressing (read-only) |\n");
    
    // System Time
    md.push_str("\n### System Time Registers\n\n");
    md.push_str("| Register | Description |\n");
    md.push_str("|----------|-------------|\\n");
    md.push_str("| SYS_TIME_MS | 16-bit system time (milliseconds, 0-999) |\n");
    md.push_str("| SYS_TIME_S | 32-bit system time (seconds) |\n");
    
    // Keyboard/Input
    md.push_str("\n### Keyboard & Input Registers\n\n");
    md.push_str("| Register | Description |\n");
    md.push_str("|----------|-------------|\\n");
    md.push_str("| KEY_IO | 8-bit key state (read-only, 0=pressed) |\n");
    md.push_str("| SYS_KEY_COUNT | 32-bit physical key press count (read-only) |\n");
    md.push_str("| SYS_KEY_LAY | 8-bit keyboard layer |\n");
    md.push_str("| SYS_KBLED | 8-bit keyboard LED status (Num/Caps/Scroll Lock) |\n");
    md.push_str("| HE_KEY_LV | 32-bit magnetic axis key depth (micrometers) |\n");
    
    // LED Control
    md.push_str("\n### LED Control Registers\n\n");
    md.push_str("| Register | Description |\n");
    md.push_str("|----------|-------------|\\n");
    md.push_str("| SELECTED_LED | 8-bit selected LED |\n");
    md.push_str("| SELECTED_LED_COL | 24-bit selected LED color (RGB888) |\n");
    md.push_str("| ALL_LED_COL | 24-bit all LED colors (RGB888) |\n");
    
    // System Control
    md.push_str("\n### System Control Registers\n\n");
    md.push_str("| Register | Description |\n");
    md.push_str("|----------|-------------|\\n");
    md.push_str("| SCRIPT_ADDR | 32-bit script starting address (read-only) |\n");
    md.push_str("| CFG_ADDR | 32-bit configuration file address (read-only) |\n");
    md.push_str("| RANDOM | 32-bit random number (read/set seed) |\n");
    md.push_str("| SYS_BLE_NUM | 8-bit Bluetooth device switching |\n");
    md.push_str("| SYS_VOLUME | 8-bit system volume |\n");
    md.push_str("| SYS_USB_SUSP | 8-bit USB sleep status |\n");
    md.push_str("| ZERO | 8-bit constant 0 (read-only) |\n");
    md.push_str("| GL_SIZE | 8-bit number of global registers (read-only) |\n");
    
    md
}

/// Format operands for display
fn format_operands(operands: &[OperandDef]) -> String {
    if operands.is_empty() {
        return "-".to_string();
    }
    
    operands.iter()
        .enumerate()
        .map(|(_i, op)| {
            let type_str = match op.op_type {
                OperandType::None => "-",
                OperandType::Register => "reg",
                OperandType::Label => "label",
                OperandType::U8 => "u8",
                OperandType::I8 => "i8",
                OperandType::U16 => "u16",
                OperandType::I16 => "i16",
                OperandType::U32 => "u32",
                OperandType::I32 => "i32",
                OperandType::Rgb888 => "rgb888",
            };
            format!("{}", type_str)
        })
        .collect::<Vec<_>>()
        .join(", ")
}
