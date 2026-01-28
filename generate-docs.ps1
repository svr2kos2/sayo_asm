#!/usr/bin/env pwsh
# 自动生成Sayo汇编文档

Write-Host "🔨 生成Sayo汇编文档..." -ForegroundColor Cyan
Write-Host ""

# 运行文档生成器
cargo run --example generate_docs

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ 文档生成成功！" -ForegroundColor Green
    Write-Host ""
    Write-Host "生成的文档：" -ForegroundColor Yellow
    Write-Host "  📄 docs/instructions.md - 指令集参考" -ForegroundColor White
    Write-Host "  📄 docs/registers.md - 寄存器参考" -ForegroundColor White
    Write-Host ""
    
    # 显示文件大小
    $instrFile = Get-Item "docs/instructions.md"
    $regFile = Get-Item "docs/registers.md"
    Write-Host "文件信息：" -ForegroundColor Yellow
    Write-Host "  instructions.md: $($instrFile.Length) bytes" -ForegroundColor Gray
    Write-Host "  registers.md: $($regFile.Length) bytes" -ForegroundColor Gray
} else {
    Write-Host ""
    Write-Host "❌ 文档生成失败" -ForegroundColor Red
    exit 1
}
