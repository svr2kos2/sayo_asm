use sayo_parser::parse;

#[test]
fn test_parse_simple_instruction() {
    let input = "mov R0, 10\n";
    let result = parse(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_with_label() {
    let input = r#"
main:
    mov R0, SYS_KEY_COUNT
    RET
"#;
    let result = parse(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_with_directives() {
    let input = r#"
    .text
    .globl main
    .align 1
main:
    mov R0, 10
    RET
"#;
    let result = parse(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_arithmetic() {
    let input = r#"
    ADD R0, R1
    SUB R2, R3
    MUL_A
    DIV_A
"#;
    let result = parse(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_jumps() {
    let input = r#"
    JMP main
    SJMP 10
    CALL function
    RET
"#;
    let result = parse(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_parse_press_release() {
    let input = r#"
    PRESS_GK_VAL R0
    SLEEP 10
    RELEASE_GK_VAL R0
"#;
    let result = parse(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}
