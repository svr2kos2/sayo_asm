# Sayo Assembly Language Support

完整的 Sayo 汇编语言工具链，包含解析器、汇编器、LSP 服务器和 VS Code 扩展。

## 🎉 最新更新

**2026-01-08**: ✅ **汇编器 MVP 完成！** 现在可以将汇编代码编译成机器码并在硬件上运行！

## 项目结构

```
sayo_asm/
├── crates/
│   ├── sayo_ast/           # AST 定义（寄存器、指令、语法树）
│   ├── sayo_parser/        # 词法分析器和语法解析器
│   ├── sayo_sema/          # 语义分析器
│   ├── sayo_assembler/     # 🆕 汇编器核心库
│   ├── sayoasm/            # 🆕 命令行汇编工具
│   └── sayo_lsp/           # Language Server Protocol 服务器
├── vscode-extension/       # VS Code 扩展
│   ├── src/extension.ts    # 扩展入口
│   ├── syntaxes/           # TextMate 语法高亮规则
│   └── package.json        # 扩展配置
├── docs/                   # 📚 完整文档
│   ├── assembler_mvp_plan.md          # MVP 实现计划
│   ├── assembler_implementation.md    # 技术实现总结
│   ├── assembler_usage.md             # 使用指南
│   └── COMPLETION_SUMMARY.md          # 完成总结
└── vs_test/                # 测试用汇编文件
```

## ✅ 已完成功能

### 🆕 阶段 4: 汇编器 (完成 ✓)

- **核心功能**
  - ✅ Layout Pass（地址分配、符号表生成）
  - ✅ 全局/局部标签作用域管理
  - ✅ Section 支持（.text / .data）
  - ✅ Directive 处理（.align, .byte, .word, .long, .ascii, .zero, 等）
  - ✅ 完整的指令编码（寄存器、立即数、标签）
  - ✅ PC 相对跳转自动计算（SJMP 等）
  - ✅ Listing 生成（LLVM 风格 show-encoding 格式）

- **CLI 工具**
  - ✅ `sayoasm` - 命令行汇编器
  - ✅ 二进制输出（.bin）
  - ✅ Listing 输出（.lst，带地址和编码注释）

**快速开始**：
```bash
# 构建汇编器
cargo build --release

# 汇编代码
./target/release/sayoasm input.s -o output.bin -l

# 查看生成的 listing
cat input.lst
```

**示例输出**：
```
main:                                    ; addr: 0x0000
	MOV8 R0, 10                             ; addr: 0x0000  encoding: [0x6f,0x04,0x0a]
	ADD8 R0, 5                              ; addr: 0x0003  encoding: [0x73,0x04,0x05]
	RET                                      ; addr: 0x0006  encoding: [0x55]
```

详细文档：[docs/assembler_usage.md](docs/assembler_usage.md)

### 阶段 3: 语义分析 (完成 ✓)

- **语义检查器**
  - ✅ 标签定义和引用验证
  - ✅ 操作数类型检查
  - ✅ 寄存器读写权限检查

### 阶段 1: 核心解析器 (完成 ✓)

- **AST 定义**
  - 80+ 寄存器支持 (V0-V3, R0-R15, 系统寄存器, GL_0-GL_63)
  - 150+ 指令助记符
  - 完整的 AST 结构 (Program, Item, Directive, Instruction, Operand)

- **词法分析器**
  - Token 识别：指令、寄存器、整数、十六进制、注释、标签
  - 支持 peek_token() 前瞻

- **语法解析器**
  - 手动实现（绕过 LALRPOP 的 LR(1) 冲突）
  - 支持指令解析 (`.text`, `.data`, `.globl`, `.align`, `.file`, `.type`, `.size`)
  - 解析汇编指令和操作数
  - 测试覆盖：8个测试全部通过

- **测试验证**
  - ✅ 成功解析 main_encoding.s (619项：144指令 + 475汇编指令)

### 阶段 2: VS Code 插件与 LSP (完成 ✓)

- **语义分析器** (crates/sayo_sema/) 🆕
  - 立即数范围检查 (u8, i8, u16, i16, u32, i32)
  - 只读寄存器写入检查
  - 操作数数量检查
  - 操作数类型检查
  - **标签作用域和跳转验证** 🔥
    - 全局标签 vs 局部标签（`.` 前缀）
    - 两遍扫描算法（第一遍收集定义，第二遍验证引用）
    - 作用域隔离（不同函数的同名局部标签互不冲突）
    - 未定义标签检测
    - 重复标签检测
    - 局部标签作用域错误检测
    - 支持立即数地址（0-65535）作为标签替代
  - 14 个单元测试全部通过

- **LSP 服务器** (crates/sayo_lsp/)
  - Tower-LSP 框架实现
  - 文档同步 (打开/修改)
  - 自动补全 (指令、寄存器、指令)
  - 悬停提示
  - 语法诊断
  - **语义诊断** 🆕
    - 实时错误检测（立即数范围、只读寄存器、操作数错误）
    - **标签错误实时提示**（未定义、重复、作用域错误）

- **VS Code 扩展** (vscode-extension/)
  - TextMate 语法高亮
    - 注释高亮 (`;`)
    - 指令高亮 (`.text`, `.data`, `.globl` 等)
    - 标签高亮 (`label:`)
    - 控制流指令高亮 (`jmp`, `call`, `ret`)
    - 通用指令高亮 (`mov`, `add`, `sub`)
    - I/O 指令高亮 (`PRESS_GK`, `RELEASE_GK`, `SLEEP`)
    - 寄存器高亮 (`R0-R15`, `V0-V3`, `SYS_*`, `GL_*`)
    - 数字高亮 (十进制、十六进制、负数)
  - Language Server 集成
  - 自动补全支持
  - 配置选项

## 编译与安装

### 1. 编译 LSP 服务器

```bash
cd d:\workspace\sayo_asm
cargo build --release -p sayo-lsp
```

编译后的可执行文件位于：`target/release/sayo-lsp.exe`

### 2. 安装 VS Code 扩展

```bash
cd vscode-extension
npm install
npm run compile
```

### 3. 配置 VS Code

在 VS Code 设置中配置 LSP 服务器路径：

```json
{
  "sayo-asm.languageServer.path": "d:\\workspace\\sayo_asm\\target\\release\\sayo-lsp.exe"
}
```

或者扩展会自动查找工作区中的 `target/debug/sayo-lsp.exe`。

### 4. 开发模式运行扩展

1. 在 VS Code 中打开 `vscode-extension` 文件夹
2. 按 `F5` 启动扩展开发主机
3. 打开 `.s` 或 `.asm` 文件测试

## 使用示例

### 基本示例

创建 `test.s` 文件：

```assembly
    .text
    .globl main
    .align 1
main:
    mov R0, 10        ; 移动立即数到寄存器
    PRESS_GK_VAL R0   ; 按下游戏键
    SLEEP 100         ; 延迟100ms
    RELEASE_GK_VAL R0 ; 释放游戏键
    RET               ; 返回
```

### 标签作用域示例 🆕

```assembly
; 全局标签和局部标签示例
main:
    CALL init
    CALL process
    JMP end

init:
    MOV R0, 0
.loop:              ; 局部标签，属于 init
    INC R0
    JNZ R0, .loop   ; 跳转到 init 的 .loop
    RET

process:
    MOV R1, 10
.loop:              ; 另一个 .loop，属于 process（不冲突！）
    DEC R1
    JNZ R1, .loop   ; 跳转到 process 的 .loop
    RET

end:
    END
```

**注意**：
- `init` 和 `process` 都有自己的 `.loop` 标签，互不冲突
- 全局标签（`main`, `init`, `process`）可以跨函数引用
- 局部标签（`.loop`）只在当前函数作用域内有效
- 也可以使用直接地址：`JMP 0` 或 `JMP 1000`

### 错误示例

语义检查器会捕获这些错误：

```assembly
test:
    SLEEP 300           ; ❌ 错误: 300 超出 u8 范围 (0-255)
    MOV8 ZERO, 10       ; ❌ 错误: ZERO 是只读寄存器
    ADD_R R0, R1        ; ❌ 错误: ADD_R 需要 3 个操作数
    JMP undefined       ; ❌ 错误: 未定义的标签 'undefined'
    
.orphan:                ; ❌ 错误: 局部标签没有全局标签上下文
    NOP
```

扩展将提供：
- 🎨 **语法高亮**: 指令、寄存器、注释自动着色
- 💡 **自动补全**: 输入 `.` 或指令前缀时显示建议
- 🔍 **悬停提示**: 鼠标悬停显示指令说明
- ❌ **语法错误诊断**: 实时检测语法错误
- ⚠️ **语义错误诊断**: 实时检测语义错误（范围、类型、标签等） 🆕

## 测试

```bash
# 运行所有测试
cargo test

# 测试解析器
cargo test -p sayo_parser

# 测试语义分析器（包含标签作用域测试）
cargo test -p sayo_sema

# 解析特定文件
cargo run --example parse_file -- main_encoding.s
```

**测试文件**：
- `test_complete_metadata.s` - 完整的指令 metadata 测试
- `test_label_scope.s` - 标签作用域正确使用示例 🆕
- `test_label_errors.s` - 标签错误示例 🆕
- `test_semantic.s` - 综合语义检查示例

## 架构说明

### 解析器流程

```
源代码 (.s)
    ↓
词法分析器 (Lexer)
    ↓
Token 流
    ↓
语法解析器 (Parser)
    ↓
AST (抽象语法树)
```

### LSP 通信

```
VS Code Extension
    ↓ (stdio)
LSP Server (sayo-lsp)
    ↓
Parser + AST
    ↓
返回补全/诊断/悬停信息
```

## 下一步（可选）

### 阶段 3: 汇编器 (未实现)

- 指令编码
- 二进制生成
- 重定位支持
- 符号表管理

## 依赖

- **Rust**: 1.70+
- **Node.js**: 20+
- **VS Code**: 1.80+

## Rust 依赖项

- `tower-lsp`: LSP 服务器框架
- `tokio`: 异步运行时
- `serde`: 序列化/反序列化
- `lalrpop-util`: 解析器工具（预留）

## 许可

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

## 已知限制

1. 标签解析当前被识别为指令（待优化）
2. 错误消息位置信息不够精确
3. 部分高级指令的操作数验证未实现
4. main_encoding.s 文件编码问题（已通过字节读取解决）

##更新日志

### v0.1.0 (2026-01-07)

- ✅ 完成核心解析器
- ✅ 完成 LSP 服务器
- ✅ 完成 VS Code 扩展基础功能
- ✅ 成功解析 619 行真实汇编代码
