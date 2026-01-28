# 自动文档生成功能实现总结

## 实现概述

我们在 `sayo_ast` crate 中添加了自动文档生成功能，可以从代码中提取指令和寄存器的元数据，并生成格式化的Markdown文档。

## 实现的功能

### 1. 在 `crates/sayo_ast/src/lib.rs` 中新增的函数

#### `generate_instruction_markdown()`
- 遍历所有指令助记符（通过 `Mnemonic::all_variants()`）
- 提取每个指令的元数据（操作码、长度、操作数等）
- 生成完整的指令集参考表格
- 按类别分组显示指令：
  - 控制流
  - 延时
  - 算术运算
  - 逻辑运算
  - 数据传送
  - HID键盘/鼠标/多媒体/手柄
  - 系统与LED控制
  - 内存与线程管理
  - 打印与调试

#### `generate_register_markdown()`
- 遍历所有寄存器（通过 `Register::common_variants()`）
- 提取每个寄存器的元数据（索引、位宽、读写属性等）
- 生成完整的寄存器参考表格
- 包含GL_0到GL_63全局寄存器
- 按类别分组显示寄存器：
  - 通用寄存器
  - 间接寻址寄存器
  - 系统时间寄存器
  - 键盘与输入寄存器
  - LED控制寄存器
  - 系统控制寄存器

#### `format_operands()`
辅助函数，用于格式化操作数类型为可读的字符串。

### 2. 示例程序 `crates/sayo_ast/examples/generate_docs.rs`

创建了一个简单的命令行工具，可以运行来生成文档：

```bash
cargo run --example generate_docs
```

该工具会：
1. 调用文档生成函数
2. 将结果写入 `docs/instructions.md` 和 `docs/registers.md`
3. 自动创建 docs 目录（如果不存在）
4. 显示生成成功的消息

### 3. 生成的文档

#### `docs/instructions.md`
- 包含143条指令的完整参考
- 每条指令都有：助记符、操作码(Hex)、指令长度、操作数、作用、备注
- 按操作码排序的主表格
- 按功能类别分组的详细说明

#### `docs/registers.md`
- 包含所有寄存器的完整参考
- 每个寄存器都有：名称、索引(Hex)、位宽、读写属性、说明
- 包含64个全局寄存器的完整列表
- 按功能类别分组的详细说明

#### `docs/README.md`
说明文档，解释如何使用和重新生成文档。

## 技术优势

1. **单一数据源**：文档直接从代码中生成，确保文档与代码实现保持同步
2. **自动化**：无需手动维护CSV文件和文档的对应关系
3. **可维护性**：当添加新指令或寄存器时，只需更新枚举和元数据，运行生成器即可更新文档
4. **类型安全**：利用Rust的类型系统确保所有指令和寄存器都被正确记录
5. **格式一致**：所有文档使用统一的Markdown表格格式

## 使用方法

### 生成文档
```bash
cd d:\workspace\sayo_asm
cargo run --example generate_docs
```

### 查看文档
- [docs/instructions.md](docs/instructions.md) - 指令集参考
- [docs/registers.md](docs/registers.md) - 寄存器参考

## 未来改进建议

1. 添加指令使用示例
2. 生成HTML版本的文档
3. 添加搜索功能
4. 生成PDF版本
5. 添加中英文双语支持
6. 集成到CI/CD流程，自动检测文档是否过期

## 相关文件

- `crates/sayo_ast/src/lib.rs` - 核心实现
- `crates/sayo_ast/src/instr.rs` - 指令元数据
- `crates/sayo_ast/src/reg.rs` - 寄存器元数据
- `crates/sayo_ast/examples/generate_docs.rs` - 文档生成器
- `docs/instructions.md` - 生成的指令文档
- `docs/registers.md` - 生成的寄存器文档
- `docs/README.md` - 文档说明
