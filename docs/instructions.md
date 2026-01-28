# Sayo Assembly Instruction Reference

Auto-generated documentation containing all Sayo assembly instructions.

## Instruction List

| Mnemonic | Opcode(Hex) | Length | Operands | Description | Notes |
|--------|-------------|----------|--------|------|------|
| END | 0x00 | 1 | - | End program |  |
| NOP | 0x01 | 1 | - | No operation |  |
| JMP | 0x02 | 3 | label | PC = i; | Long jump |
| SJMP | 0x03 | 2 | i8 | PC = PC + i; | Short jump, offset |
| AJMP | 0x04 | 2 | u8 | PC = (PC & 0xff00) + i; | Jump within 256B range of address |
| SLEEP_X256 | 0x05 | 2 | u8 | Sleep(i * 256); | Delay range 0-65280ms (256x multiplier) |
| SLEEP | 0x06 | 2 | u8 | Sleep(i * 1); | Delay range 0-255ms |
| SLEEP_RAND_X256 | 0x07 | 2 | u8 | Sleep(rand()%(i * 256)+1); | Random delay range 1-65281ms (256x multiplier) |
| SLEEP_RAND | 0x08 | 2 | u8 | Sleep(rand()%i+1); | Random delay range 1-256ms |
| SLEEP_X256_VAL | 0x09 | 2 | reg | Sleep(i * 256); | Register version of delay, range depends on register (256x multiplier) |
| SLEEP_VAL | 0x0A | 2 | reg | Sleep(i); | Register version of delay, range depends on register |
| SLEEP_RAND_X8_VAL | 0x0B | 2 | reg | Sleep(rand()%(i * 8)+1); | Register version of random delay, range depends on register (8x multiplier) |
| SLEEP_RAND_VAL | 0x0C | 2 | reg | Sleep(rand()%i+1); | Register version of random delay, range depends on register |
| SLEEP_U16 | 0x0D | 3 | u16 | Sleep(i); | Delay range 1-65536ms |
| SLEEP_RAND_U16 | 0x0E | 3 | u16 | Sleep(rand()%i+1); | Delay range 1-65536ms |
| PRESS_SK | 0x10 | 2 | u8 | Keyboard modifier key i pressed | HID keycode |
| PRESS_GK | 0x11 | 2 | u8 | Keyboard normal key i pressed | HID keycode |
| PRESS_MK | 0x12 | 2 | u8 | Mouse button i pressed | HID keycode |
| PRESS_MU | 0x13 | 2 | u8 | Media key i pressed | HID keycode |
| PRESS_SK_VAL | 0x14 | 2 | reg | Keyboard modifier key i pressed | Register version |
| PRESS_GK_VAL | 0x15 | 2 | reg | Keyboard normal key i pressed | Register version |
| PRESS_MK_VAL | 0x16 | 2 | reg | Mouse button i pressed | Register version |
| PRESS_MU_VAL | 0x17 | 2 | reg | Media key i pressed | Register version |
| RELEASE_SK | 0x18 | 2 | u8 | Keyboard modifier key i released | HID keycode |
| RELEASE_GK | 0x19 | 2 | u8 | Keyboard normal key i released | HID keycode |
| RELEASE_MK | 0x1A | 2 | u8 | Mouse button i released | HID keycode |
| RELEASE_MU | 0x1B | 2 | u8 | Media key i released | HID keycode |
| RELEASE_SK_VAL | 0x1C | 2 | reg | Keyboard modifier key i released | Register version |
| RELEASE_GK_VAL | 0x1D | 2 | reg | Keyboard normal key i released | Register version |
| RELEASE_MK_VAL | 0x1E | 2 | reg | Mouse button i released | Register version |
| RELEASE_MU_VAL | 0x1F | 2 | reg | Media key i released | Register version |
| UPDATE | 0x20 | 1 | - | Force HID packet retransmission | Rarely used |
| MO_XYZ | 0x21 | 3 | u8, i8 | Mouse cursor movement axis=i data=j | 0: x, 1:y, 2:scroll |
| MO_XYZ_VAL | 0x22 | 3 | u8, reg | Mouse cursor movement axis=i data=j | Register version |
| GA_XYZ | 0x23 | 4 | u8, u16 | joystick axis=i data=j |  |
| GA_XYZ_VAL | 0x24 | 3 | u8, reg | joystick axis=i data=j | Register version |
| TB_XY | 0x25 | 5 | i16, i16 | Mouse cursor positioning x=i y=j |  |
| TB_XY_VAL | 0x26 | 3 | reg, reg | Mouse cursor positioning x=i y=j | Register version |
| DIAL_DATA | 0x27 | 2 | u8 | Dial data=i | data:0=release 1=press 2=cw 3=ccw |
| DIAL_DATA_VAL | 0x28 | 2 | reg | Dial data=i | data:0=release 1=press 2=cw 3=ccw, register version |
| KEY_TO_AXIS | 0x29 | 1 | - | joystick axis=reg::val[0] type=reg::val[1] | Internal use |
| PRESS_GAK | 0x2C | 2 | u8 | joystick button i pressed |  |
| PRESS_GAK_VAL | 0x2D | 2 | reg | joystick button i pressed | Register version |
| RELEASE_GAK | 0x2E | 2 | u8 | joystick button i released |  |
| RELEASE_GAK_VAL | 0x2F | 2 | reg | joystick button i released | Register version |
| C2K | 0x30 | 1 | - | print ascii character | Internal use |
| U2K | 0x31 | 1 | - | print unicode character | Internal use |
| C2K_RAND | 0x32 | 1 | - | print random ascii character | Internal use |
| U2K_REG | 0x33 | 1 | - | print value | Internal use, requires looping until complete |
| PRINT_REG | 0x34 | 2 | reg | print value | Print register value, single execution outputs complete |
| JFA | 0x40 | 4 | reg, reg, reg | if (i>j)PC=k; | Compare unsigned (unsigned), jump if true. Target in register |
| JFB | 0x41 | 4 | reg, reg, reg | if (i<j)PC=k; | Compare unsigned (unsigned), jump if true. Target in register |
| JFG | 0x42 | 4 | reg, reg, reg | if (i>j)PC=k; | Compare unsigned (unsigned), jump if true. Target in register |
| JFL | 0x43 | 4 | reg, reg, reg | if (i<j)PC=k; | Compare unsigned (unsigned), jump if true. Target in register |
| JA | 0x44 | 5 | reg, reg, label | if (i>j)PC=k; | Compare signed (>), jump if true. Target is label |
| JB | 0x45 | 5 | reg, reg, label | if (i<j)PC=k; | Compare signed (<), jump if true. Target is label |
| JG | 0x46 | 5 | reg, reg, label | if (i>j)PC=k; | Compare signed (>), jump if true. Target is label |
| JL | 0x47 | 5 | reg, reg, label | if (i<j)PC=k; | Compare signed (<), jump if true. Target is label |
| JFC | 0x48 | 2 | reg | if (CY) PC = i; | Jump if CY set. Target address stored in register |
| JFNC | 0x49 | 2 | reg | if (!CY) PC = i; | Jump if CY not set. Target address stored in register |
| JFZ | 0x4A | 3 | reg, reg | if (!i) PC = j; | Jump if register is 0. Target address stored in register |
| JFNZ | 0x4B | 3 | reg, reg | if (i) PC = j; | Jump if register is not 0. Target address stored in register |
| DJFNZ | 0x4C | 3 | reg, reg | if (--i) PC = j; | Decrement and jump if not 0. Target address stored in register |
| CJFNE | 0x4D | 4 | reg, reg, reg | if (i != j) {CY = i<j;PC = k} | Compare and set CY, jump if not equal. Target in register |
| JC | 0x4E | 3 | label | if (CY) PC = i; | Jump if CY set. Target address is label |
| JNC | 0x4F | 3 | label | if (!CY) PC = i; | Jump if CY not set. Target address is label |
| JZ | 0x50 | 4 | reg, label | if (!i) PC = j; | Jump if register is 0. Target address is label |
| JNZ | 0x51 | 4 | reg, label | if (i) PC = j; | Jump if register is not 0. Target address is label |
| DJNZ | 0x52 | 4 | reg, label | if (--i) PC = j; | Decrement and jump if not 0. Target address is label |
| CJNE | 0x53 | 5 | reg, reg, label | if (i != j) {CY = i<j;PC = k} | Compare and set CY, jump if not equal. Target is label |
| CALL | 0x54 | 3 | label | PUSH PC;PC=i; | Call subroutine, target address is label |
| RET | 0x55 | 1 | - | POP PC; | Subroutine return |
| AND | 0x56 | 3 | reg, reg | i=i&j; | Bitwise AND |
| AND8 | 0x57 | 3 | reg, u8 | i=i&j; | 8-bit width |
| ADD_A | 0x58 | 2 | reg | A = A + i; | Add to A register |
| ADD8_A | 0x59 | 2 | u8 | A = A + i; | 8-bit immediate add to A |
| SUB_A | 0x5A | 2 | reg | A = A - i; | Subtract from A register |
| SUB8_A | 0x5B | 2 | u8 | A = A - i; | 8-bit immediate subtract from A |
| OR_A | 0x5C | 2 | reg | A = A | i; | Bitwise OR to A register |
| OR8_A | 0x5D | 2 | u8 | A = A | i; | 8-bit immediate bitwise OR to A |
| DEC | 0x5E | 2 | reg | i--; | Decrement |
| INC | 0x5F | 2 | reg | i++; | Increment |
| MUL_A | 0x60 | 1 | - | A = A * B; | Multiplication, result stored in A |
| DIV_A | 0x61 | 1 | - | A = A / B;B = A % B; | Division, quotient in A, remainder in B |
| XOR | 0x62 | 3 | reg, reg | i=i^j; | Bitwise XOR |
| XOR8 | 0x63 | 3 | reg, u8 | i=i^j; | 8-bit width |
| SHL | 0x64 | 3 | reg, reg | i=i<<j; | Logical left shift |
| SHL8 | 0x65 | 3 | reg, u8 | i=i<<j; | 8-bit immediate |
| SHR | 0x66 | 3 | reg, reg | i=i>>j; | Logical right shift |
| SHR8 | 0x67 | 3 | reg, u8 | i=i>>j; | 8-bit immediate |
| CLR | 0x68 | 2 | reg | i=0; | Clear register |
| NOT | 0x69 | 2 | reg | i=~i; | Bitwise NOT |
| XCH | 0x6A | 3 | reg, reg | i <=> j; | Exchange values of two registers |
| CMP | 0x6B | 3 | reg, reg | CY=i<j; | Compare two registers and set CY flag |
| PUSH | 0x6C | 2 | reg |  | Push to stack |
| POP | 0x6D | 2 | reg |  | Pop from stack |
| MOV | 0x6E | 3 | reg, reg | i=j; |  |
| MOV8 | 0x6F | 3 | reg, u8 | i=j; | 8-bit width |
| MOV16 | 0x70 | 4 | reg, u16 | i=j; | 16-bit width |
| MOV32 | 0x71 | 6 | reg, u32 | i=j; | 32-bit width |
| ADD | 0x72 | 3 | reg, reg | i=i+j; |  |
| ADD8 | 0x73 | 3 | reg, u8 | i=i+j; | 8-bit width |
| ADD16 | 0x74 | 4 | reg, u16 | i=i+j; | 16-bit width |
| SUB | 0x75 | 3 | reg, reg | i=i-j; |  |
| SUB8 | 0x76 | 3 | reg, u8 | i=i-j; | 8-bit width |
| SUB16 | 0x77 | 4 | reg, u16 | i=i-j; | 16-bit width |
| OR | 0x78 | 3 | reg, reg | i=i|j; | Bitwise OR |
| OR8 | 0x79 | 3 | reg, u8 | i=i|j; | 8-bit width |
| AND16 | 0x7A | 4 | reg, u16 | i=i&j; | 16-bit width |
| OR16 | 0x7B | 4 | reg, u16 | i=i|j; | 16-bit width |
| XOR16 | 0x7C | 4 | reg, u16 | i=i^j; | 16-bit width |
| ADD32 | 0x7D | 6 | reg, u32 | i=i+j; | 32-bit width |
| SUB32 | 0x7E | 6 | reg, u32 | i=i-j; | 32-bit width |
| AND32 | 0x7F | 6 | reg, u32 | i=i&j; | 32-bit width |
| OR32 | 0x80 | 6 | reg, u32 | i=i|j; | 32-bit width |
| XOR32 | 0x81 | 6 | reg, u32 | i=i^j; | 32-bit width |
| ADD_R | 0x82 | 4 | reg, reg, reg | i=j+k; | Three-operand addition |
| SUB_R | 0x83 | 4 | reg, reg, reg | i=j-k; | Three-operand subtraction |
| AND_R | 0x84 | 4 | reg, reg, reg | i=j&k; | Three-operand bitwise AND |
| OR_R | 0x85 | 4 | reg, reg, reg | i=j|k; | Three-operand bitwise OR |
| XOR_R | 0x86 | 4 | reg, reg, reg | i=j^k; | Three-operand bitwise XOR |
| MUL_R | 0x87 | 4 | reg, reg, reg | i=j*k; | Three-operand multiplication |
| DIV_R | 0x88 | 4 | reg, reg, reg | i=j/k; | Three-operand division |
| MOD_R | 0x89 | 4 | reg, reg, reg | i=j%k; | Three-operand modulo |
| MOVSX8b | 0x8A | 3 | reg, reg | i = sign_extend(j); | 8-bit sign extend to target width |
| MOVSX16b | 0x8B | 3 | reg, reg | i = sign_extend(j); | 16-bit sign extend to target width |
| MOV8SX | 0x8C | 3 | reg, i8 | i = sign_extend(imm8); | 8-bit immediate sign extend |
| MOV16SX | 0x8D | 4 | reg, i16 | i = sign_extend(imm16); | 16-bit immediate sign extend |
| IMUL_A | 0x8E | 1 | - | A=A*B; | Signed multiplication, result stored in A |
| IMUL_R | 0x8F | 4 | reg, reg, reg | i=j*k; | Three-operand signed multiplication |
| LED_CTRL | 0xE0 | 2 | u8 | SELECTED_LED = i; | 0xff = release |
| LED_COL | 0xE1 | 4 | rgb888 | SELECTED_LED_COL = i; | RGB888 format |
| START | 0xE2 | 2 | u8 | Start_key(i-1); | 0=all |
| STOP | 0xE3 | 2 | u8 | Stop_key(i-1); | 0=all |
| SYCON | 0xE8 | 2 | u8 |  | System control |
| MALLOC | 0xF0 | 2 | reg | i=malloc(i); |  |
| FREE | 0xF1 | 2 | reg | i=free(i); |  |
| NEW_THREAD | 0xF2 | 4 | u8, reg, reg | i=TH ID;j=addr or keymode;k=V[4] | Range of i is 0~3 |
| WHILE_UPDATE | 0xF4 | 1 | - | while (update_flag)Sleep(1); | Wait for HID upload complete |
| JMP_TO_SCRIPT | 0xF5 | 2 | u8 |  | Jump to other script (register data preserved, PC reset) |
| MOV_PC2REG | 0xF6 | 2 | reg | i=PC; | Save next instruction address to register |
| VALUE_RELOAD | 0xF7 | 2 | reg | i=Reload(reg); | Reload script parameters |
| MODE_JOG | 0xF8 | 1 | - |  | Enter jog mode (key press won't be forcibly interrupted) |
| WAIT_IF_RELEASE | 0xF9 | 1 | - | while (IO) Sleep(1); | If physical key is released, wait for press |
| WAIT_IF_PRESS | 0xFA | 1 | - | while (!IO) Sleep(1); | If physical key is pressed, wait for release |
| EXIT_IF_RELEAS | 0xFB | 1 | - | if (IO) exit(); | Exit if physical key is released |
| EXIT_IF_PRESS | 0xFC | 1 | - | if (!IO) exit(); | Exit if physical key is pressed |
| EXIT_IF_ANYKEY | 0xFD | 1 | - | if (SYS_KEY_COUNT != n) exit(); | n=key counter at script start. Exit on any key press |
| RES | 0xFE | 1 | - | PC = 0; | Jump to program start, same as JMP 0 |
| EXIT | 0xFF | 1 | - | exit(); | Exit script |

## By Category

### Control Flow Instructions

| Instruction | Description |
|-------------|-------------|
| END | End program |
| NOP | No operation |
| JMP | PC = i; (Long jump) |
| SJMP | PC = PC + i; (Short jump, offset) |
| AJMP | PC = (PC & 0xff00) + i; (Jump within 256B range of address) |
| CALL | PUSH PC;PC=i; (Call subroutine, target address is label) |
| RET | POP PC; (Subroutine return) |
| JZ | if (!i) PC = j; (Jump if register is 0. Target address is label) |
| JNZ | if (i) PC = j; (Jump if register is not 0. Target address is label) |
| JC | if (CY) PC = i; (Jump if CY set. Target address is label) |
| JNC | if (!CY) PC = i; (Jump if CY not set. Target address is label) |
| DJNZ | if (--i) PC = j; (Decrement and jump if not 0. Target address is label) |
| JA | if (i>j)PC=k; (Compare signed (>), jump if true. Target is label) |
| JB | if (i<j)PC=k; (Compare signed (<), jump if true. Target is label) |
| JG | if (i>j)PC=k; (Compare signed (>), jump if true. Target is label) |
| JL | if (i<j)PC=k; (Compare signed (<), jump if true. Target is label) |
| CJNE | if (i != j) {CY = i<j;PC = k} (Compare and set CY, jump if not equal. Target is label) |
| RES | PC = 0; (Jump to program start, same as JMP 0) |
| EXIT | exit(); (Exit script) |

### Delay Instructions

| Instruction | Description |
|-------------|-------------|
| SLEEP | Sleep(i * 1); (Delay range 0-255ms) |
| SLEEP_X256 | Sleep(i * 256); (Delay range 0-65280ms (256x multiplier)) |
| SLEEP_U16 | Sleep(i); (Delay range 1-65536ms) |
| SLEEP_RAND | Sleep(rand()%i+1); (Random delay range 1-256ms) |
| SLEEP_RAND_X256 | Sleep(rand()%(i * 256)+1); (Random delay range 1-65281ms (256x multiplier)) |
| SLEEP_RAND_U16 | Sleep(rand()%i+1); (Delay range 1-65536ms) |
| SLEEP_VAL | Sleep(i); (Register version of delay, range depends on register) |
| SLEEP_X256_VAL | Sleep(i * 256); (Register version of delay, range depends on register (256x multiplier)) |
| SLEEP_RAND_VAL | Sleep(rand()%i+1); (Register version of random delay, range depends on register) |
| SLEEP_RAND_X8_VAL | Sleep(rand()%(i * 8)+1); (Register version of random delay, range depends on register (8x multiplier)) |

### Arithmetic Instructions

| Instruction | Description |
|-------------|-------------|
| ADD | i=i+j; |
| ADD8 | i=i+j; (8-bit width) |
| ADD16 | i=i+j; (16-bit width) |
| ADD32 | i=i+j; (32-bit width) |
| SUB | i=i-j; |
| SUB8 | i=i-j; (8-bit width) |
| SUB16 | i=i-j; (16-bit width) |
| SUB32 | i=i-j; (32-bit width) |
| MUL_A | A = A * B; (Multiplication, result stored in A) |
| DIV_A | A = A / B;B = A % B; (Division, quotient in A, remainder in B) |
| IMUL_A | A=A*B; (Signed multiplication, result stored in A) |
| ADD_R | i=j+k; (Three-operand addition) |
| SUB_R | i=j-k; (Three-operand subtraction) |
| MUL_R | i=j*k; (Three-operand multiplication) |
| DIV_R | i=j/k; (Three-operand division) |
| MOD_R | i=j%k; (Three-operand modulo) |
| IMUL_R | i=j*k; (Three-operand signed multiplication) |
| INC | i++; (Increment) |
| DEC | i--; (Decrement) |
| ADD_A | A = A + i; (Add to A register) |
| ADD8_A | A = A + i; (8-bit immediate add to A) |
| SUB_A | A = A - i; (Subtract from A register) |
| SUB8_A | A = A - i; (8-bit immediate subtract from A) |

### Logical Instructions

| Instruction | Description |
|-------------|-------------|
| AND | i=i&j; (Bitwise AND) |
| AND8 | i=i&j; (8-bit width) |
| AND16 | i=i&j; (16-bit width) |
| AND32 | i=i&j; (32-bit width) |
| OR | i=i|j; (Bitwise OR) |
| OR8 | i=i|j; (8-bit width) |
| OR16 | i=i|j; (16-bit width) |
| OR32 | i=i|j; (32-bit width) |
| XOR | i=i^j; (Bitwise XOR) |
| XOR8 | i=i^j; (8-bit width) |
| XOR16 | i=i^j; (16-bit width) |
| XOR32 | i=i^j; (32-bit width) |
| NOT | i=~i; (Bitwise NOT) |
| SHL | i=i<<j; (Logical left shift) |
| SHL8 | i=i<<j; (8-bit immediate) |
| SHR | i=i>>j; (Logical right shift) |
| SHR8 | i=i>>j; (8-bit immediate) |
| AND_R | i=j&k; (Three-operand bitwise AND) |
| OR_R | i=j|k; (Three-operand bitwise OR) |
| XOR_R | i=j^k; (Three-operand bitwise XOR) |
| OR_A | A = A | i; (Bitwise OR to A register) |
| OR8_A | A = A | i; (8-bit immediate bitwise OR to A) |

### Data Transfer Instructions

| Instruction | Description |
|-------------|-------------|
| MOV | i=j; |
| MOV8 | i=j; (8-bit width) |
| MOV16 | i=j; (16-bit width) |
| MOV32 | i=j; (32-bit width) |
| MOVSX8b | i = sign_extend(j); (8-bit sign extend to target width) |
| MOVSX16b | i = sign_extend(j); (16-bit sign extend to target width) |
| MOV8SX | i = sign_extend(imm8); (8-bit immediate sign extend) |
| MOV16SX | i = sign_extend(imm16); (16-bit immediate sign extend) |
| XCH | i <=> j; (Exchange values of two registers) |
| PUSH |  (Push to stack) |
| POP |  (Pop from stack) |
| CLR | i=0; (Clear register) |
| CMP | CY=i<j; (Compare two registers and set CY flag) |

### HID Keyboard Instructions

| Instruction | Description |
|-------------|-------------|
| PRESS_SK | Keyboard modifier key i pressed (HID keycode) |
| PRESS_GK | Keyboard normal key i pressed (HID keycode) |
| RELEASE_SK | Keyboard modifier key i released (HID keycode) |
| RELEASE_GK | Keyboard normal key i released (HID keycode) |
| PRESS_SK_VAL | Keyboard modifier key i pressed (Register version) |
| PRESS_GK_VAL | Keyboard normal key i pressed (Register version) |
| RELEASE_SK_VAL | Keyboard modifier key i released (Register version) |
| RELEASE_GK_VAL | Keyboard normal key i released (Register version) |

### HID Mouse Instructions

| Instruction | Description |
|-------------|-------------|
| PRESS_MK | Mouse button i pressed (HID keycode) |
| RELEASE_MK | Mouse button i released (HID keycode) |
| PRESS_MK_VAL | Mouse button i pressed (Register version) |
| RELEASE_MK_VAL | Mouse button i released (Register version) |
| MO_XYZ | Mouse cursor movement axis=i data=j (0: x, 1:y, 2:scroll) |
| MO_XYZ_VAL | Mouse cursor movement axis=i data=j (Register version) |
| TB_XY | Mouse cursor positioning x=i y=j |
| TB_XY_VAL | Mouse cursor positioning x=i y=j (Register version) |

### HID Media & Gamepad Instructions

| Instruction | Description |
|-------------|-------------|
| PRESS_MU | Media key i pressed (HID keycode) |
| RELEASE_MU | Media key i released (HID keycode) |
| PRESS_MU_VAL | Media key i pressed (Register version) |
| RELEASE_MU_VAL | Media key i released (Register version) |
| PRESS_GAK | joystick button i pressed |
| RELEASE_GAK | joystick button i released |
| PRESS_GAK_VAL | joystick button i pressed (Register version) |
| RELEASE_GAK_VAL | joystick button i released (Register version) |
| GA_XYZ | joystick axis=i data=j |
| GA_XYZ_VAL | joystick axis=i data=j (Register version) |
| DIAL_DATA | Dial data=i (data:0=release 1=press 2=cw 3=ccw) |
| DIAL_DATA_VAL | Dial data=i (data:0=release 1=press 2=cw 3=ccw, register version) |
| UPDATE | Force HID packet retransmission (Rarely used) |

### System & LED Control Instructions

| Instruction | Description |
|-------------|-------------|
| LED_CTRL | SELECTED_LED = i; (0xff = release) |
| LED_COL | SELECTED_LED_COL = i; (RGB888 format) |
| START | Start_key(i-1); (0=all) |
| STOP | Stop_key(i-1); (0=all) |
| SYCON |  (System control) |
| JMP_TO_SCRIPT |  (Jump to other script (register data preserved, PC reset)) |
| MODE_JOG |  (Enter jog mode (key press won't be forcibly interrupted)) |
| WAIT_IF_RELEASE | while (IO) Sleep(1); (If physical key is released, wait for press) |
| WAIT_IF_PRESS | while (!IO) Sleep(1); (If physical key is pressed, wait for release) |
| EXIT_IF_RELEAS | if (IO) exit(); (Exit if physical key is released) |
| EXIT_IF_PRESS | if (!IO) exit(); (Exit if physical key is pressed) |
| EXIT_IF_ANYKEY | if (SYS_KEY_COUNT != n) exit(); (n=key counter at script start. Exit on any key press) |
| WHILE_UPDATE | while (update_flag)Sleep(1); (Wait for HID upload complete) |

### Memory & Threading Instructions

| Instruction | Description |
|-------------|-------------|
| MALLOC | i=malloc(i); |
| FREE | i=free(i); |
| NEW_THREAD | i=TH ID;j=addr or keymode;k=V[4] (Range of i is 0~3) |
| MOV_PC2REG | i=PC; (Save next instruction address to register) |
| VALUE_RELOAD | i=Reload(reg); (Reload script parameters) |

### Print & Debug Instructions

| Instruction | Description |
|-------------|-------------|
| PRINT_REG | print value (Print register value, single execution outputs complete) |
| C2K | print ascii character (Internal use) |
| U2K | print unicode character (Internal use) |
| C2K_RAND | print random ascii character (Internal use) |
| U2K_REG | print value (Internal use, requires looping until complete) |
