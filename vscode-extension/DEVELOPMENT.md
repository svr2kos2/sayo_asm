# Sayo Assembly VS Code Extension - 开发指南

## 打包步骤

1. **编译 LSP 服务器**:
   ```powershell
   cd d:\workspace\sayo_asm
   cargo build -p sayo_lsp --release
   ```

2. **编译扩展并打包**:
   ```powershell
   cd vscode-extension
   npm install
   npm run package
   ```
   
   这会:
   - 编译 TypeScript
   - 自动复制 `target/release/sayo-lsp.exe` 到 `bin/` 目录
   - 创建 `.vsix` 包,包含 exe

3. **安装打包的扩展**:
   ```
   code --install-extension sayo-asm-0.1.0.vsix
   ```

## 开发模式

在开发时,你可以:
1. 在 settings.json 中配置开发路径(可选):
   ```json
   "sayo-asm.languageServer.path": "d:\\workspace\\sayo_asm\\target\\debug\\sayo-lsp.exe"
   ```

2. 或者直接在工作区开发,扩展会自动找到 `target/debug/sayo-lsp.exe`

## LSP 服务器查找优先级

1. **用户配置** (`sayo-asm.languageServer.path`) - 开发时使用
2. **打包的 exe** (`extension/bin/sayo-lsp.exe`) - 发布版本
3. **工作区编译** (`workspace/target/{release,debug}/sayo-lsp.exe`) - 开发模式

## 解决 exe 被锁定问题

如果 `sayo-lsp.exe` 被 VS Code 锁定无法重新编译:

### 方法 1: 重新加载窗口 (推荐)
- 按 `Ctrl+Shift+P`
- 输入 "Reload Window"
- 这会重启 VS Code 窗口但保留状态

### 方法 2: 停止语言服务器
- 按 `Ctrl+Shift+P`
- 输入 "Restart Extension Host"

### 方法 3: 使用不同的构建目录
在开发时使用 debug 版本,发布时编译 release:
```powershell
# 开发时
cargo build -p sayo_lsp

# 配置指向 debug 版本
"sayo-asm.languageServer.path": "d:\\workspace\\sayo_asm\\target\\debug\\sayo-lsp.exe"

# 打包时编译 release
cargo build -p sayo_lsp --release
```

### 方法 4: 临时禁用扩展
在 settings.json 中设置不存在的路径,强制扩展启动失败:
```json
"sayo-asm.languageServer.path": "C:\\nonexistent\\path.exe"
```

## 发布给其他用户

打包后的 `.vsix` 文件已经包含了 `sayo-lsp.exe`,其他用户:
1. 只需安装 `.vsix` 文件
2. 不需要安装 Rust 或编译任何东西
3. 扩展会自动使用打包的 exe
