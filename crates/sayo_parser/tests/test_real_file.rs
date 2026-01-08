use sayo_parser::parse;

#[test]
fn test_parse_complex_assembly() {
    // This is a sample similar to main_encoding.s structure
    let input = r#"
    .text
    .file   "main.c"
    .globl  sys_key_count
    .align  1
    .type   sys_key_count,@function
sys_key_count:
    mov R0, SYS_KEY_COUNT
    RET
.Lfunc_end0:
    .size   sys_key_count, .Lfunc_end0-sys_key_count
    .globl  print_reg
    .align  1
    .type   print_reg,@function
print_reg:
    PRINT_REG R0
    RET
.Lfunc_end1:
    .size   print_reg, .Lfunc_end1-print_reg
    .globl  press_and_release_gk
    .align  1
    .type   press_and_release_gk,@function
press_and_release_gk:
    AND8 R0, 255
    PRESS_GK_VAL R0
    SLEEP 10
    RELEASE_GK_VAL R0
    SLEEP 10
    RET
.Lfunc_end2:
    .size   press_and_release_gk, .Lfunc_end2-press_and_release_gk
"#;
    
    let result = parse(&input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    
    let program = result.unwrap();
    println!("Successfully parsed {} items", program.items.len());
    
    // Verify we have the expected structure
    assert!(program.items.len() > 20, "Expected more than 20 items, got {}", program.items.len());
    
    // Print first few items for verification
    for (i, item) in program.items.iter().take(15).enumerate() {
        println!("{}: {:?}", i, item);
    }
}
