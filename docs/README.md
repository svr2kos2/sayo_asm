# Sayo汇编文档

本目录包含自动生成的Sayo汇编参考文档。

## 文档列表

- [instructions.md](instructions.md) - 指令集参考手册
- [registers.md](registers.md) - 寄存器参考手册

## 重新生成文档

要重新生成文档，请运行：

```bash
cargo run --example generate_docs
```

这将自动从代码中提取指令和寄存器的元数据，并生成最新的Markdown文档。

## 文档特性

### 指令集文档 (instructions.md)

- 完整的指令列表，按操作码排序
- 包含助记符、操作码、指令长度、操作数类型、作用和备注
- 按类别分组：
  - 控制流指令
  - 延时指令
  - 算术运算指令
  - 逻辑运算指令
  - 数据传送指令
  - HID键盘指令
  - HID鼠标指令
  - HID多媒体与手柄指令
  - 系统与LED控制指令
  - 内存与线程管理指令
  - 打印与调试指令

### 寄存器文档 (registers.md)

- 完整的寄存器列表
- 包含寄存器名称、索引、位宽、读写属性和说明
- 全局寄存器GL_0到GL_63的完整列表
- 按类别分组：
  - 通用寄存器
  - 间接寻址寄存器
  - 系统时间寄存器
  - 键盘与输入寄存器
  - LED控制寄存器
  - 系统控制寄存器

## 实现细节

文档生成功能在 `crates/sayo_ast/src/lib.rs` 中实现：

- `generate_instruction_markdown()` - 生成指令集文档
- `generate_register_markdown()` - 生成寄存器文档

这些函数遍历所有指令和寄存器的枚举类型，提取元数据并格式化为Markdown表格。

示例代码在 `crates/sayo_ast/examples/generate_docs.rs` 中。
