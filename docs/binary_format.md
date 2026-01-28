# Sayo Binary Format Specification

## Version: 1.0

## Overview

Sayo 汇编器生成的 `.bin` 文件采用以下布局，支持代码段和数据段的分离，便于反汇编和调试。

## Binary Layout

```
┌─────────────────────────────────────┐ 0x0000
│  CALL main                          │  3 bytes (opcode 0x54 + u16 addr)
├─────────────────────────────────────┤ 0x0003
│  EXIT                               │  1 byte  (opcode 0xFF)
├─────────────────────────────────────┤ 0x0004
│  Header                             │  8 bytes
│  ├─ magic: "SAYO"                   │  4 bytes (0x53, 0x41, 0x59, 0x4F)
│  ├─ version: u8                     │  1 byte  (当前版本: 0x01)
│  ├─ text_size: u16 LE               │  2 bytes (代码段大小)
│  └─ reserved: u8                    │  1 byte  (保留，填 0x00)
├─────────────────────────────────────┤ 0x000C (offset = 12)
│  .text section                      │  text_size bytes
│  (main 函数入口在此段内)             │
├─────────────────────────────────────┤ 0x000C + text_size
│  .data section                      │
│  ├─ data_count: u16 LE              │  2 bytes (数据块数量)
│  ├─ len[0]: u16 LE                  │  2 bytes (第一个数据块长度)
│  ├─ data[0]: [u8; len[0]]           │  len[0] bytes
│  ├─ len[1]: u16 LE                  │  2 bytes
│  ├─ data[1]: [u8; len[1]]           │
│  └─ ...                             │
└─────────────────────────────────────┘
```

## Header Details

| Offset | Size | Field       | Description                           |
|--------|------|-------------|---------------------------------------|
| 0x0000 | 3    | call_main   | `CALL <main_addr>` 指令，调用 main    |
| 0x0003 | 1    | exit        | `EXIT` 指令，当 main 返回时退出程序   |
| 0x0004 | 4    | magic       | ASCII "SAYO" (0x53 0x41 0x59 0x4F)   |
| 0x0008 | 1    | version     | 格式版本号，当前为 0x01               |
| 0x0009 | 2    | text_size   | 代码段大小 (little-endian u16)        |
| 0x000B | 1    | reserved    | 保留字节，填充 0x00                   |

**Total Header Size: 12 bytes (0x000C)**

## Section Details

### .text Section

- 起始地址: `0x000C` (紧跟 header)
- 大小: `text_size` bytes (从 header 读取)
- 内容: 机器码指令
- 注意: `main` 标签的地址会被写入 JMP 指令

### .data Section

- 起始地址: `0x000B + text_size`
- 格式: 结构化数据块列表

```
┌─────────────────────────────────────┐
│ data_count: u16 LE                  │  数据块数量 (0-65535)
├─────────────────────────────────────┤
│ Block 0:                            │
│   ├─ length: u16 LE                 │  数据长度 (0-65535 bytes)
│   └─ data: [u8; length]             │  实际数据
├─────────────────────────────────────┤
│ Block 1:                            │
│   ├─ length: u16 LE                 │
│   └─ data: [u8; length]             │
├─────────────────────────────────────┤
│ ...                                 │
└─────────────────────────────────────┘
```

## Address Calculation

代码中引用数据标签时，地址计算公式：

```
data_label_addr = HEADER_SIZE + text_size + data_offset
                = 11 + text_size + data_offset
```

其中 `data_offset` 是该数据在 .data section 内的偏移（包含 data_count 和 length 字节）。

## Example

假设有以下汇编：

```asm
.text
.globl main
main:
    MOV32 R0, message    ; 加载 message 的地址
    RET

.data
message:
    .asciz "Hello"       ; 6 bytes (含 \0)
count:
    .byte 42             ; 1 byte
```

生成的二进制：

```
Offset  Bytes                         Description
------  -----                         -----------
0x0000  54 00 0C                      CALL 0x000C (main)
0x0003  FF                            EXIT
0x0004  53 41 59 4F                   Magic "SAYO"
0x0008  01                            Version 1
0x0009  06 00                         text_size = 6
0x000B  00                            Reserved
0x000C  70 04 13 00 00 00             MOV32 R0, 0x0013 (message addr)
0x0012  55                            RET
0x0013  02 00                         data_count = 2 (u16 LE)
0x0014  06 00                         len[0] = 6 (u16 LE)
0x0016  48 65 6C 6C 6F 00             "Hello\0"
0x001C  01 00                         len[1] = 1 (u16 LE)
0x001E  2A                            42
```

## Validation

反汇编器/加载器应验证：

1. Magic 字段必须为 "SAYO"
2. Version 必须为支持的版本号
3. `text_size` 不应超过文件剩余大小
4. 数据段解析不应越界

## CLI Options

```bash
# 生成完整格式（带 header）
sayoasm input.s -o output.bin

# 生成原始格式（无 header，仅用于调试）
sayoasm input.s -o output.bin --raw

# 指定入口点（默认为 main）
sayoasm input.s -o output.bin --entry start
```

## Limitations

- 代码段最大: 65535 bytes (u16)
- 数据块数量最大: 65535 个 (u16)
- 单个数据块最大: 65535 bytes (u16)
- 必须存在 `main` 标签，否则汇编失败

## Future Extensions

- Version 2 可扩展 header 支持更多元数据
- 可添加符号表 section
- 可添加调试信息 section
