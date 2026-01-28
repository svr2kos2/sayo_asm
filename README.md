# Sayo Assembly Language

Sayo assembly language compiler toolchain.  
Also check [llvm for sayo](https://github.com/svr2kos2/llvm_sayo_backend)

## Release Files

- **clang.exe** - Custom Clang compiler for Sayo
- **clang++.exe** - Custom Clang++ compiler for Sayo
- **sayo-asm-0.1.0.vsix** - VS Code syntax highlighting and language support extension
- **sayoasm.exe** - Assembly compiler (.s → .bin/.lst)
- **sayo_uploader.exe** - Firmware upload tool

## Quick Start

### 0. Download Release Files
There is a example folder and another README.md in release files.  
You can get start from there more directly and quickly.  


### 1. Install VS Code Extension (Optional)

In VS Code:
1. Press `Ctrl+Shift+P`
2. Type "Install from VSIX"
3. Select `sayo-asm-0.1.0.vsix` file
4. Restart VS Code

Now `.s` and `.asm` files will have syntax highlighting.

### 2. Write Assembly Code or Get from Compiler

Create `program.s`:

```asm
    .text
    .globl  main
main:
    MOV8 R0, 10
    ADD8 R0, 5
    RET
```

### 3. Compile to Machine Code

```bash
sayoasm program.s -o program.bin -l
```

**Options:**
- `-o <file>` - Output binary file (required)
- `-l` - Generate listing file

Generated files:
- `program.bin` - Executable machine code
- `program.lst` - Machine code listing with addresses and encodings

**Listing example:**
```
main:                                    ; addr: 0x0000
    MOV8 R0, 10                          ; addr: 0x0000  encoding: [0x6f,0x04,0x0a]
    ADD8 R0, 5                           ; addr: 0x0003  encoding: [0x73,0x04,0x05]
    RET                                  ; addr: 0x0006  encoding: [0x55]
```

### 4. Upload to Device

```bash
sayo_uploader program.bin
```

## Syntax Guide

### Labels

```asm
main:              ; Global label
.loop:             ; Local label (belongs to the nearest global label)
    MOV8 R0, 1
    SJMP .loop
```

### Instructions and Operands

```asm
MOV8 R0, 10        ; Register, immediate
ADD8 R0, R1        ; Register, register
CALL function      ; Label
SJMP .loop         ; Relative jump
```

### Directives

> **Note:** Directive support is incomplete. Some directives are ignored entirely, and `.data` does not yet support complex data structures. If you encounter issues, there's a high probability they stem from limitations in this project.

```asm
.text              ; Code section
.data              ; Data section
.globl main        ; Global symbol
.align 4           ; Align to 4-byte boundary
.byte 1, 2, 3      ; Byte data
.ascii "Hello"     ; ASCII string
.zero 10           ; Fill with 10 zero bytes
```

## License

MIT License
