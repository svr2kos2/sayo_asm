# Sayo ASM 汇编器使用指南

## 安装

```bash
# 构建 release 版本
cargo build --release

# 可执行文件位置
# Windows: target/release/sayoasm.exe
# Linux/macOS: target/release/sayoasm
```

## 基本使用

### 命令格式

```bash
sayoasm <input.s> -o <output.bin> [OPTIONS]
```

### 选项

- `-o, --output <FILE>` - 输出二进制文件路径
- `-l, --listing` - 生成 listing 文件（带编码注释）
- `--listing-output <FILE>` - 自定义 listing 文件路径（默认：`<input>.lst`）

### 示例

```bash
# 1. 仅生成二进制
sayoasm program.s -o program.bin

# 2. 生成二进制 + listing
sayoasm program.s -o program.bin -l

# 3. 自定义 listing 路径
sayoasm program.s -o program.bin -l --listing-output program.lst
```

## 汇编语法

### 基本结构

```asm
	.text                    ; 代码段
	.globl	main             ; 全局符号声明
main:                        ; 全局标签
	MOV8 R0, 10             ; 指令
	CALL function           ; 调用
	RET                     ; 返回

function:
.loop:                       ; 局部标签（归属 function）
	ADD8 R0, 1
	SJMP .loop              ; 跳转到局部标签
	RET
```

### 支持的 Directives

#### Section 控制
- `.text` - 切换到代码段
- `.data` - 切换到数据段

#### 符号声明
- `.globl <name>` - 声明全局符号
- `.type <name>, @function` - 符号类型（可选）
- `.size <name>, <expr>` - 符号大小（可选）

#### 对齐
- `.align <N>` - 对齐到 N 字节边界
- `.p2align <N>` - 对齐到 2^N 字节边界

#### 数据定义
- `.byte <values>` - 1 字节数据，例如 `.byte 1, 2, 3`
- `.word <values>` - 2 字节数据，例如 `.word 0x1234`
- `.long <values>` - 4 字节数据，例如 `.long 0x12345678`
- `.quad <values>` - 8 字节数据
- `.ascii "<string>"` - ASCII 字符串
- `.zero <count>` - 填充 N 个零字节
- `.skip <count>` - 跳过 N 个字节

### 标签规则

#### 全局标签
- 不带 `.` 前缀
- 在整个程序中可见
- 示例：`main:`, `function:`, `data_start:`

#### 局部标签
- 带 `.` 前缀
- 作用域：最近的全局标签
- 示例：`.loop:`, `.LBB0_1:`, `.Lfunc_end0:`

### 指令操作数

#### 寄存器
```asm
MOV8 R0, R1              ; 通用寄存器 R0-R15
ADD8 V0, 1               ; 参数寄存器 V0-V3
MOV DPTR, R4             ; 特殊寄存器
```

#### 立即数
```asm
MOV8 R0, 10              ; 十进制
MOV8 R1, 0xFF            ; 十六进制
ADD8 R2, -5              ; 负数
```

#### 标签
```asm
SJMP loop                ; 相对跳转（自动计算偏移）
CALL function            ; 绝对地址
AJMP 0x100               ; 绝对跳转
```

## Listing 文件格式

生成的 `.lst` 文件格式：

```
<source_line>                            ; addr: 0xADDR  encoding: [0xXX,0xYY,...]
```

示例：

```
	.text                                   ; addr: 0x0000
	.globl	main                             ; addr: 0x0000
main:                                    ; addr: 0x0000
	MOV8 R0, 10                             ; addr: 0x0000  encoding: [0x6f,0x04,0x0a]
	ADD8 R0, 5                              ; addr: 0x0003  encoding: [0x73,0x04,0x05]
	RET                                      ; addr: 0x0006  encoding: [0x55]
```

## 常见问题

### 1. 未定义的标签

**错误**：
```
Assembly error: Undefined label: loop
```

**解决**：确保标签已定义，且局部标签前有全局标签。

### 2. 立即数超出范围

**错误**：
```
Assembly error: Immediate value 300 out of range for 8-bit operand
```

**解决**：检查指令的操作数类型，使用合适的指令变体（如 MOV16 代替 MOV8）。

### 3. PC 相对偏移超出范围

**错误**：
```
Assembly error: PC-relative offset 200 out of range for 8-bit signed
```

**解决**：目标太远，使用绝对跳转或重新组织代码。

## 完整示例

### 输入：test.s

```asm
	.text
	.globl	main

main:
	MOV8 R0, 0           ; 初始化计数器
	MOV8 R1, 10          ; 设置上限

.loop:
	ADD8 R0, 1           ; 递增
	CMP R0, R1           ; 比较
	SJMP .loop           ; 循环
	RET                  ; 返回

	.data
data_buffer:
	.byte 1, 2, 3, 4, 5
	.zero 10
```

### 运行

```bash
sayoasm test.s -o test.bin -l
```

### 输出：test.lst

```
	.text                                   ; addr: 0x0000
	.globl	main                             ; addr: 0x0000

main:                                    ; addr: 0x0000
	MOV8 R0, 0                              ; addr: 0x0000  encoding: [0x6f,0x04,0x00]
	MOV8 R1, 10                             ; addr: 0x0003  encoding: [0x6f,0x05,0x0a]

.loop:                                   ; addr: 0x0006
	ADD8 R0, 1                              ; addr: 0x0006  encoding: [0x73,0x04,0x01]
	CMP R0, R1                              ; addr: 0x0009  encoding: [0x4e,0x04,0x05]
	SJMP .loop                              ; addr: 0x000b  encoding: [0x03,0xf9]
	RET                                      ; addr: 0x000d  encoding: [0x55]

	.data                                   ; addr: 0x0000
data_buffer:                             ; addr: 0x0000
	.byte 1, 2, 3, 4, 5                     ; addr: 0x0000  encoding: [0x01,0x02,0x03,0x04,0x05]
	.zero 10                                ; addr: 0x0005  encoding: [0x00,0x00,0x00,0x00,0x00,...]
```

### 输出：test.bin（十六进制）

```
6F 04 00 6F 05 0A 73 04 01 4E 04 05 03 F9 55 01 02 03 04 05 00 00 00 00 00 00 00 00 00 00
```

## 技术支持

如遇问题，请检查：
1. 语法是否符合 Sayo ASM 规范
2. 所有标签是否已定义
3. 立即数范围是否合法
4. Directive 是否受支持

更多技术细节请参考：`docs/assembler_implementation.md`
