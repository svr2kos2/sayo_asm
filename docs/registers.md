# Sayo Assembly Register Reference

Auto-generated documentation containing all Sayo assembly registers.

## Register List

| Register | Index(Hex) | Width | Access | Description |
|--------|-----------|------|------|------|
| V0 | 0x00 | 8-bit | RW | Key parameters/General purpose register |
| V1 | 0x01 | 8-bit | RW | Key parameters/General purpose register |
| V2 | 0x02 | 8-bit | RW | Key parameters/General purpose register |
| V3 | 0x03 | 8-bit | RW | Key parameters/General purpose register |
| R0 | 0x04 | 32-bit | RW | General purpose register |
| R1 | 0x05 | 32-bit | RW | General purpose register |
| R2 | 0x06 | 32-bit | RW | General purpose register |
| R3 | 0x07 | 32-bit | RW | General purpose register |
| R4 | 0x20 | 32-bit | RW | General purpose register |
| R5 | 0x21 | 32-bit | RW | General purpose register |
| R6 | 0x22 | 32-bit | RW | General purpose register |
| R7 | 0x23 | 32-bit | RW | General purpose register |
| R8 | 0x24 | 32-bit | RW | General purpose register |
| R9 | 0x25 | 32-bit | RW | General purpose register |
| R10 | 0x26 | 32-bit | RW | General purpose register |
| R11 | 0x27 | 32-bit | RW | General purpose register |
| R12 | 0x28 | 32-bit | RW | General purpose register |
| R13 | 0x29 | 32-bit | RW | General purpose register |
| R14 | 0x2A | 32-bit | RW | General purpose register |
| R15 | 0x2B | 32-bit | RW | General purpose register |
| DPTR | 0x09 | 32-bit | RW | Mapped to R4 |
| *DPTR | 0x08 | 8-bit | R | ROM addressing dedicated register, mapped to R4, shared address space |
| KEY_IO | 0x0A | 8-bit | R | 0=pressed |
| ZERO | 0x0F | 8-bit | R | Always reads as 0 |
| A | 0x10 | 32-bit | RW | Dedicated register. Mapped to R6, shared address space. Can reduce code length for certain instructions |
| B | 0x11 | 32-bit | RW | Dedicated register. Mapped to R7, shared address space. Can reduce code length for certain instructions |
| *R0 | 0x0B | 8-bit | RW | RAM addressing using register (8-bit) |
| *R1 | 0x0C | 8-bit | RW | RAM addressing using register (8-bit) |
| *R2 | 0x0D | 8-bit | RW | RAM addressing using register (8-bit) |
| *R3 | 0x0E | 8-bit | RW | RAM addressing using register (8-bit) |
| *R4 | 0x2C | 8-bit | RW | RAM addressing using register (8-bit) |
| *R5 | 0x2D | 8-bit | RW | RAM addressing using register (8-bit) |
| *R6 | 0x2E | 8-bit | RW | RAM addressing using register (8-bit) |
| *R7 | 0x2F | 8-bit | RW | RAM addressing using register (8-bit) |
| *R0_8b | 0x0B | 8-bit | RW | RAM addressing using register (8-bit) |
| *R1_8b | 0x0C | 8-bit | RW | RAM addressing using register (8-bit) |
| *R2_8b | 0x0D | 8-bit | RW | RAM addressing using register (8-bit) |
| *R3_8b | 0x0E | 8-bit | RW | RAM addressing using register (8-bit) |
| *R4_8b | 0x2C | 8-bit | RW | RAM addressing using register (8-bit) |
| *R5_8b | 0x2D | 8-bit | RW | RAM addressing using register (8-bit) |
| *R6_8b | 0x2E | 8-bit | RW | RAM addressing using register (8-bit) |
| *R7_8b | 0x2F | 8-bit | RW | RAM addressing using register (8-bit) |
| *R0_16b | 0x30 | 16-bit | RW | RAM addressing using register (16-bit) |
| *R1_16b | 0x31 | 16-bit | RW | RAM addressing using register (16-bit) |
| *R2_16b | 0x32 | 16-bit | RW | RAM addressing using register (16-bit) |
| *R3_16b | 0x33 | 16-bit | RW | RAM addressing using register (16-bit) |
| *R4_16b | 0x34 | 16-bit | RW | RAM addressing using register (16-bit) |
| *R5_16b | 0x35 | 16-bit | RW | RAM addressing using register (16-bit) |
| *R6_16b | 0x36 | 16-bit | RW | RAM addressing using register (16-bit) |
| *R7_16b | 0x37 | 16-bit | RW | RAM addressing using register (16-bit) |
| *R0_32b | 0x38 | 32-bit | RW | RAM addressing using register (32-bit) |
| *R1_32b | 0x39 | 32-bit | RW | RAM addressing using register (32-bit) |
| *R2_32b | 0x3A | 32-bit | RW | RAM addressing using register (32-bit) |
| *R3_32b | 0x3B | 32-bit | RW | RAM addressing using register (32-bit) |
| *R4_32b | 0x3C | 32-bit | RW | RAM addressing using register (32-bit) |
| *R5_32b | 0x3D | 32-bit | RW | RAM addressing using register (32-bit) |
| *R6_32b | 0x3E | 32-bit | RW | RAM addressing using register (32-bit) |
| *R7_32b | 0x3F | 32-bit | RW | RAM addressing using register (32-bit) |
| SYS_TIME_MS | 0x12 | 16-bit | R | System time in milliseconds. Range 0-999 |
| SYS_TIME_S | 0x13 | 32-bit | R | System time in seconds |
| SYS_KBLED | 0x14 | 8-bit | RW | Keyboard LED status (Num Lock, Caps Lock, Scroll Lock, etc.) |
| SYS_KEY_COUNT | 0x15 | 32-bit | R | Physical key press count |
| SYS_KEY_LAY | 0x16 | 8-bit | RW | Keyboard layer. A keyboard may have multiple key layers |
| SCRIPT_ADDR | 0x17 | 32-bit | R | Script starting address |
| RANDOM | 0x18 | 32-bit | RW | R: Get random number W: Set random seed |
| SYS_BLE_NUM | 0x19 | 8-bit | RW | Bluetooth multi-device switching |
| SYS_VOLUME | 0x1A | 8-bit | RW | Absolute system volume; currently ineffective on Windows |
| SELECTED_LED | 0x1B | 8-bit | RW | Selected LED for operation. Default is the LED of the pressed key |
| SELECTED_LED_COL | 0x1C | 24-bit | RW | Modify the color of selected LED (RGB888) |
| ALL_LED_COL | 0x1D | 24-bit | RW | Modify the color of all LEDs (RGB888) |
| CFG_ADDR | 0x1E | 32-bit | R | Get current configuration file address |
| HE_KEY_LV | 0x1F | 32-bit | RW | Magnetic axis key depth value in micrometers |
| SYS_USB_SUSP | 0x40 | 8-bit | RW | R: 1=USB in sleep state W: Wake up host |
| GL_SIZE | 0x7F | 8-bit | R | Number of GL registers (minimum 4, maximum 64) |

### Global Registers (GL_0 to GL_63)

Global registers GL_0 to GL_63 are 32-bit read-write registers for global data storage.
The number of available registers can be read from GL_SIZE register (minimum 4, maximum 64).

| Register | Index(Hex) | Width | Access |
|--------|-----------|------|------|
| GL_0 | 0x80 | 32-bit | RW |
| GL_1 | 0x81 | 32-bit | RW |
| GL_2 | 0x82 | 32-bit | RW |
| GL_3 | 0x83 | 32-bit | RW |
| GL_4 | 0x84 | 32-bit | RW |
| GL_5 | 0x85 | 32-bit | RW |
| GL_6 | 0x86 | 32-bit | RW |
| GL_7 | 0x87 | 32-bit | RW |
| GL_8 | 0x88 | 32-bit | RW |
| GL_9 | 0x89 | 32-bit | RW |
| GL_10 | 0x8A | 32-bit | RW |
| GL_11 | 0x8B | 32-bit | RW |
| GL_12 | 0x8C | 32-bit | RW |
| GL_13 | 0x8D | 32-bit | RW |
| GL_14 | 0x8E | 32-bit | RW |
| GL_15 | 0x8F | 32-bit | RW |
| GL_16 | 0x90 | 32-bit | RW |
| GL_17 | 0x91 | 32-bit | RW |
| GL_18 | 0x92 | 32-bit | RW |
| GL_19 | 0x93 | 32-bit | RW |
| GL_20 | 0x94 | 32-bit | RW |
| GL_21 | 0x95 | 32-bit | RW |
| GL_22 | 0x96 | 32-bit | RW |
| GL_23 | 0x97 | 32-bit | RW |
| GL_24 | 0x98 | 32-bit | RW |
| GL_25 | 0x99 | 32-bit | RW |
| GL_26 | 0x9A | 32-bit | RW |
| GL_27 | 0x9B | 32-bit | RW |
| GL_28 | 0x9C | 32-bit | RW |
| GL_29 | 0x9D | 32-bit | RW |
| GL_30 | 0x9E | 32-bit | RW |
| GL_31 | 0x9F | 32-bit | RW |
| GL_32 | 0xA0 | 32-bit | RW |
| GL_33 | 0xA1 | 32-bit | RW |
| GL_34 | 0xA2 | 32-bit | RW |
| GL_35 | 0xA3 | 32-bit | RW |
| GL_36 | 0xA4 | 32-bit | RW |
| GL_37 | 0xA5 | 32-bit | RW |
| GL_38 | 0xA6 | 32-bit | RW |
| GL_39 | 0xA7 | 32-bit | RW |
| GL_40 | 0xA8 | 32-bit | RW |
| GL_41 | 0xA9 | 32-bit | RW |
| GL_42 | 0xAA | 32-bit | RW |
| GL_43 | 0xAB | 32-bit | RW |
| GL_44 | 0xAC | 32-bit | RW |
| GL_45 | 0xAD | 32-bit | RW |
| GL_46 | 0xAE | 32-bit | RW |
| GL_47 | 0xAF | 32-bit | RW |
| GL_48 | 0xB0 | 32-bit | RW |
| GL_49 | 0xB1 | 32-bit | RW |
| GL_50 | 0xB2 | 32-bit | RW |
| GL_51 | 0xB3 | 32-bit | RW |
| GL_52 | 0xB4 | 32-bit | RW |
| GL_53 | 0xB5 | 32-bit | RW |
| GL_54 | 0xB6 | 32-bit | RW |
| GL_55 | 0xB7 | 32-bit | RW |
| GL_56 | 0xB8 | 32-bit | RW |
| GL_57 | 0xB9 | 32-bit | RW |
| GL_58 | 0xBA | 32-bit | RW |
| GL_59 | 0xBB | 32-bit | RW |
| GL_60 | 0xBC | 32-bit | RW |
| GL_61 | 0xBD | 32-bit | RW |
| GL_62 | 0xBE | 32-bit | RW |
| GL_63 | 0xBF | 32-bit | RW |

## By Category

### General Purpose Registers

| Register | Description |
|----------|-------------|\n| V0-V3 | 8-bit parameter/general purpose registers |
| R0-R15 | 32-bit general purpose registers |
| A | 32-bit dedicated register (mapped to R6), can reduce code size for certain instructions |
| B | 32-bit dedicated register (mapped to R7), can reduce code size for certain instructions |

### Indirect Addressing Registers

| Register | Description |
|----------|-------------|\n| *R0-*R7 | 8-bit indirect addressing (using R0-R7 as address) |
| *R0_16b-*R7_16b | 16-bit indirect addressing |
| *R0_32b-*R7_32b | 32-bit indirect addressing |
| DPTR | 32-bit data pointer (mapped to R4) |
| *DPTR | 8-bit ROM addressing (read-only) |

### System Time Registers

| Register | Description |
|----------|-------------|\n| SYS_TIME_MS | 16-bit system time (milliseconds, 0-999) |
| SYS_TIME_S | 32-bit system time (seconds) |

### Keyboard & Input Registers

| Register | Description |
|----------|-------------|\n| KEY_IO | 8-bit key state (read-only, 0=pressed) |
| SYS_KEY_COUNT | 32-bit physical key press count (read-only) |
| SYS_KEY_LAY | 8-bit keyboard layer |
| SYS_KBLED | 8-bit keyboard LED status (Num/Caps/Scroll Lock) |
| HE_KEY_LV | 32-bit magnetic axis key depth (micrometers) |

### LED Control Registers

| Register | Description |
|----------|-------------|\n| SELECTED_LED | 8-bit selected LED |
| SELECTED_LED_COL | 24-bit selected LED color (RGB888) |
| ALL_LED_COL | 24-bit all LED colors (RGB888) |

### System Control Registers

| Register | Description |
|----------|-------------|\n| SCRIPT_ADDR | 32-bit script starting address (read-only) |
| CFG_ADDR | 32-bit configuration file address (read-only) |
| RANDOM | 32-bit random number (read/set seed) |
| SYS_BLE_NUM | 8-bit Bluetooth device switching |
| SYS_VOLUME | 8-bit system volume |
| SYS_USB_SUSP | 8-bit USB sleep status |
| ZERO | 8-bit constant 0 (read-only) |
| GL_SIZE | 8-bit number of global registers (read-only) |
