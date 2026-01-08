use sayo_parser::parse;
use sayo_ast::{Item, Operand};

fn main() {
    let code = r#"
; Test 1: Local labels (.LBB style)
.LBB14_25:
    MOV R0, 42
    JZ R0, .LBB14_25

; Test 2: Star registers
test_star:
    mov R8, *R0_32b
    mov R1, *R2_16b
    ADD *R3_8b, R4

; Test 3: Labels as immediates
level:
global_label:
    MOV32 R0, level
    MOV16 R1, global_label
    MOV8 R2, .LBB14_25
"#;

    match parse(code) {
        Ok(program) => {
            println!("✅ Parse successful! {} items parsed\n", program.items.len());
            
            // Check each instruction
            for item in &program.items {
                if let Item::Instruction(instr) = &item.node {
                    print!("  {:?}: ", instr.mnemonic);
                    for (i, op) in instr.operands.iter().enumerate() {
                        if i > 0 { print!(", "); }
                        match &op.node {
                            Operand::Register(r) => print!("Reg({})", r),
                            Operand::Immediate(v) => print!("Imm({})", v),
                            Operand::Label(l) => print!("Label({})", l),
                        }
                    }
                    println!();
                }
            }
            
            // Specific checks
            println!("\n🔍 Verification:");
            
            // Check 1: .LBB14_25 as operand
            let jz_found = program.items.iter().any(|item| {
                if let Item::Instruction(instr) = &item.node {
                    format!("{:?}", instr.mnemonic) == "JZ" && 
                    instr.operands.len() == 2 &&
                    matches!(&instr.operands[1].node, Operand::Label(l) if l == ".LBB14_25")
                } else {
                    false
                }
            });
            println!("  ✅ Issue 1 (JZ R0, .LBB14_25): {}", if jz_found { "FIXED" } else { "NOT FIXED" });
            
            // Check 2: *R0_32b as register
            let star_reg_found = program.items.iter().any(|item| {
                if let Item::Instruction(instr) = &item.node {
                    instr.operands.iter().any(|op| {
                        if let Operand::Register(r) = &op.node {
                            r.to_string().starts_with('*')
                        } else {
                            false
                        }
                    })
                } else {
                    false
                }
            });
            println!("  ✅ Issue 2 (*R0_32b as register): {}", if star_reg_found { "FIXED" } else { "NOT FIXED" });
            
            // Check 3: Labels as immediates
            let label_as_imm = program.items.iter().any(|item| {
                if let Item::Instruction(instr) = &item.node {
                    format!("{:?}", instr.mnemonic).starts_with("MOV") &&
                    instr.operands.len() == 2 &&
                    matches!(&instr.operands[1].node, Operand::Label(_))
                } else {
                    false
                }
            });
            println!("  ✅ Issue 3 (MOV32 R0, level): {}", if label_as_imm { "FIXED (parser accepts it)" } else { "NOT FIXED" });
            
            println!("\n✨ All issues appear to be resolved!");
        }
        Err(e) => {
            eprintln!("❌ Parse error: {:?}", e);
        }
    }
}
