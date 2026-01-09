# Sayo ASM 汇编器（MVP）实现步骤

> **状态：✅ MVP 已完成并可用！**
>
> 目标：把 LLVM 后端产出的 `.s` 真正在硬件上跑起来。
>
> 当前优先级：**先做能生成可执行二进制/镜像的汇编器**，并实现一个类似 LLVM `-show-encoding` 的文本输出：
>
> `MOV8 R2, 2    ; addr: 0x0010  encoding: [0x6f,0x60,0x40]`
>
> VS Code 的地址 inlay、unreachable、跳转箭头等功能先暂缓，等汇编器稳定后再做。

---

## 快速开始

### 安装

```bash
cargo build --release
```

可执行文件位于：`target/release/sayoasm.exe`（Windows）或 `target/release/sayoasm`（Linux/macOS）

### 使用方法

```bash
# 基本用法：汇编成二进制
sayoasm input.s -o output.bin

# 生成带编码注释的 listing 文件
sayoasm input.s -o output.bin -l

# 自定义 listing 输出路径
sayoasm input.s -o output.bin -l --listing-output output.lst
```

### 示例

```bash
# 测试简单程序
cargo run --release --bin sayoasm -- test_simple.s -o test_simple.bin -l

# 测试跳转指令
cargo run --release --bin sayoasm -- test_jump.s -o test_jump.bin -l
```

生成的 `test_jump.lst` 示例：
```
	.text                                   ; addr: 0x0000
	.globl	main                             ; addr: 0x0000
main:                                    ; addr: 0x0000
	MOV8 R0, 0                              ; addr: 0x0000  encoding: [0x6f,0x04,0x00]
loop:                                    ; addr: 0x0003
	ADD8 R0, 1                              ; addr: 0x0003  encoding: [0x73,0x04,0x01]
	SLEEP 100                               ; addr: 0x0006  encoding: [0x06,0x64]
	SJMP loop                               ; addr: 0x0008  encoding: [0x03,0xf9]
	RET                                      ; addr: 0x000a  encoding: [0x55]
```

---

## 0. MVP 范围（先把“能跑”做到极致）

### 0.1 必须支持（第一阶段）

- `.text` / `.data` 两个 section（最少 `.text` 要工作）
- 全局标签 `foo:` 与局部标签 `.Lxxx:`
  - 局部标签的作用域规则：**归属最近出现的全局标签**（和 clang/LLVM 输出风格一致）
- 指令编码：覆盖你硬件能跑的最小指令集子集（至少包含 `call/jmp/ret` 与常用 mov/alu/load/store）
- 立即数/寄存器/标签引用三种 operand
- 输出：
  1) 机器码（例如 `.bin` 或你板子加载器需要的格式）
  2) 带 `addr + encoding` 注释的 listing（用于人工肉眼验证与调试）

### 0.2 暂时不做（明确写出来，避免无限膨胀）

- 复杂表达式（例如 `.long .Lfunc_end15-main`）
- relocation / fixup 的完整体系
- 多文件链接、符号可见性与真正的链接器语义
- VS Code 增强功能（地址 inlay / CFG / unreachable / jump arrows）

> 备注：如果你的 LLVM 输出里**已经出现** `.long .Lfunc_end15-main` 这类表达式，那你有两条路：
>
> 1) **短期**：在 LLVM 侧先避免生成复杂表达式（把这些差值预先算出来或改成更简单的形式）。
> 2) **中期**：补齐表达式 AST + 两阶段回填（见后文“扩展路线”）。

---

## 1. 建议的 crate 结构（最少改动，最大复用）

你当前已有：

- `sayo_parser`：把 `.s` 解析为 AST
- `sayo_ast`：AST 数据结构、指令描述（含 `length()`）
- `sayo_sema`：语义检查（含标签分析器）

建议新增一个真正的汇编器 crate（名字可选）：

- `crates/sayo_assembler/`（或 `sayo_asm` / `sayo_codegen`）

并将逻辑拆成两个层次：

1) **Layout / Symbol**（计算地址、生成符号表、为编码准备环境）
2) **Encode**（把每条指令变成 bytes，并生成 listing）

`show-encoding` 输出属于 Encode 阶段，但它依赖 Layout 的“每条指令地址 + label 值”。

---

## 2. 输入输出形态（先定接口，后写代码）

### 2.1 输入

- `source_text: &str`
- `ast: sayo_ast::AsmFile`（由 `sayo_parser` 得到）

### 2.2 主要输出

- `Vec<u8>`：最终机器码（最少支持 `.text` 输出）
- `listing: String`：带注释的可读文本
- （调试用）
  - `symbol_table`：label → address
  - `addr_map`：每条指令/每个 AST item 的起始地址
  - `diagnostics`：错误/警告（未定义符号、立即数溢出、非法 operand 等）

---

## 3. 关键数据结构（MVP 版本）

### 3.1 地址与 section

- `Address = u32`（内部用宽一点，最终再检查是否能塞进你的 ISA 地址宽度）
- `SectionId = Text | Data`（先两个就够）

每个 section 维护一个 location counter：

- `lc_text: Address`
- `lc_data: Address`

> 注意：未来若做“把 main 段提到最前”，那是 **layout 输出顺序** 变化，不是 AST 本身乱排。

### 3.2 符号表（可复用 sayo_sema 的局部标签规则）

MVP 建议沿用你现有模型：

- `globals: HashMap<String, Address>`
- `locals: HashMap<String /*global*/, HashMap<String /*local*/, Address>>`

解析/布局时跟踪 `current_global`：

- 遇到全局标签 `foo:` → `current_global = "foo"`
- 遇到局部标签 `.L1:` → 写入 `locals[current_global][".L1"] = addr`

### 3.3 指令编码结果（用于 show-encoding）

- `EncodedInst { addr: Address, bytes: Vec<u8>, span: Span, text: String }`

`span` 用于将来 VS Code 功能；MVP 仅用于报错定位。

---

## 4. 编译流程（真正的 MVP 生产线）

### Step A：Parse

1. 调用 `sayo_parser` 把源文件解析为 `AsmFile`（保序）。
2. 如果 parser 目前不完整（如 `.ascii` 被吞掉、表达式缺失），先决定：
   - 要么扩 parser（推荐长期）
   - 要么限制 LLVM 输出（推荐短期）

### Step B：Layout Pass（地址分配 + 定义收集）

单次顺序扫描 `AsmFile.items`：

- 初始化：`section = Text`，`addr_text = 0`，`addr_data = 0`
- 对每个 item：
  - `Label(name)`：记录 `symbol[name] = current_addr(section)`
  - `Instr(inst)`：
    - 记录 `addr_map[item_id] = current_addr(section)`
    - `current_addr += inst.format.length()`（来自 `sayo_ast::instr`）
  - `Directive(dir)`：MVP 可只实现：
    - `.text` / `.data`：切 section
    - `.align N`：把 `current_addr` 按 N 对齐（定义 N 的语义：bytes 对齐还是 2^N 对齐要统一）
    - 其他：先报“暂不支持”或忽略（但忽略会让地址错，建议不支持就报错）

产物：

- `symbol_table`
- `addr_map`（每条指令/每个 item 的地址）

### Step C：Resolve Pass（引用解析）

对每条指令的 operand 做解析：

- 寄存器：直接编码
- 立即数：检查范围、符号扩展/零扩展规则
- 标签引用：
  - 若 operand 以 `.` 开头：在 `locals[current_global]` 查
  - 否则在 `globals` 查

如果找不到 → 诊断错误（带 span）。

> MVP 建议：标签引用先只支持“绝对地址”或“PC 相对小偏移”里最简单的一种。PC 相对的情况，要明确 `pc` 定义（当前指令地址还是下一条）。

### Step D：Encode Pass（真正生成 bytes）

对每条指令：

1. 取出 `addr = addr_map[item_id]`
2. 根据 opcode + operands +（必要时）label 值进行编码
3. 输出 `bytes`

把 `bytes` 追加到 `.text` 的输出缓冲区。

### Step E：生成 show-encoding listing

生成类似：

- 统一格式：
  - `原始指令文本` padding 到固定宽度
  - `; addr: 0x%04x  encoding: [0x..,0x..]`

注意点：

- listing 的“原始指令文本”最好来自源文件切片（span），而不是 AST 重新 pretty-print（否则格式/空格会变化）。
- 机器码建议按**字节**输出，因为你示例是 `[0x6f,0x60,0x40]`。

### Step F：写出最终产物

- `program.bin`：纯 `.text` bytes（或你定义的容器格式）
- `program.lst`：show-encoding listing

---

## 5. 最小化测试策略（让它尽快上硬件）

### 5.1 快速自测（软件侧）

- 用 `vs_test/main_encoding.s` 生成 `.lst`，人工 spot check：
  - label 地址递增是否合理
  - call/jmp 的目标是否匹配 label
  - 指令长度累加是否正确

### 5.2 上板验证（硬件侧）

- 先写一个最小入口程序（只做寄存器写/IO 输出/死循环）验证：
  - 指令编码正确
  - PC 流正确

然后再跑 LLVM 输出的完整程序。

---

## 6. 扩展路线（MVP 稳定后再做）

### 6.1 支持 directive 与数据布局

逐步补齐：

- `.byte/.word/.long`（影响地址与输出 bytes）
- `.ascii/.asciz`（字符串转 bytes）
- `.zero/.skip`（填充）
- `.org`（改 location counter；要定义合法性）

### 6.2 表达式与两阶段回填

支持类似：

- `.long label1 - label2`
- `imm = label + 4`

需要：

- expression AST
- layout 后 resolve
- 必要时 encode 时产生 fixup 列表并回填

### 6.3 为 VS Code 功能铺路（后续）

MVP 的 `addr_map / symbol_table / span` 都保留好，将来可以直接加：

- 行号旁地址 inlay
- 跳转箭头（branch/call 的 span → label 定义 span）
- CFG + unreachable（从 `main` 建图）

---

## 7. 交付清单（Done 的定义）

当满足以下条件，就认为 MVP 汇编器完成：

- 能成功汇编至少一个最小 `.s` 并在硬件上按预期运行
- 对 `vs_test/main_encoding.s` 能输出 `.lst`，每条指令后带 `addr` 与 `encoding`
- 遇到未定义符号/不支持 directive/立即数溢出时，能给出准确的 span 定位错误
