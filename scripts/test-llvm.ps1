#!/usr/bin/env pwsh
# LLVM 代码生成测试脚本
# Copyright © 2026 幻心梦梦 (huanxinmengmeng)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  LLVM 代码生成链路测试" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$startTime = Get-Date

function Run-Test {
    param(
        [string]$Name,
        [string]$Command
    )
    Write-Host "[$Name]" -NoNewline -ForegroundColor Yellow
    $testStart = Get-Date
    $output = Invoke-Expression $Command 2>&1
    $testElapsed = (Get-Date) - $testStart

    if ($LASTEXITCODE -eq 0) {
        Write-Host " 通过 ($([math]::Round($testElapsed.TotalMilliseconds, 2))ms)" -ForegroundColor Green
    } else {
        Write-Host " 失败 ($([math]::Round($testElapsed.TotalMilliseconds, 2))ms)" -ForegroundColor Red
        Write-Host $output | Select-Object -Last 20
        return $false
    }
    return $true
}

$allPassed = $true

Write-Host "1. 构建项目..." -ForegroundColor Magenta
$allPassed = (Run-Test "构建" "cargo build --lib 2>&1") -and $allPassed
Write-Host ""

Write-Host "2. 运行 LLVM 代码生成单元测试..." -ForegroundColor Magenta
$allPassed = (Run-Test "单元测试" "cargo test core::backend::llvm --lib 2>&1") -and $allPassed
Write-Host ""

Write-Host "3. 运行 MLIR 测试..." -ForegroundColor Magenta
$allPassed = (Run-Test "MLIR测试" "cargo test core::mlir --lib 2>&1") -and $allPassed
Write-Host ""

Write-Host "4. 运行 AST 测试..." -ForegroundColor Magenta
$allPassed = (Run-Test "AST测试" "cargo test core::ast --lib 2>&1") -and $allPassed
Write-Host ""

Write-Host "5. 检查代码覆盖率..." -ForegroundColor Magenta
$coverageCmd = "cargo test --lib 2>&1"
Write-Host "[覆盖率] 检查中..." -ForegroundColor Yellow
$coverageOutput = Invoke-Expression $coverageCmd 2>&1 | Select-String -Pattern "test result"
if ($coverageOutput) {
    Write-Host $coverageOutput -ForegroundColor Gray
}
Write-Host ""

Write-Host "6. 验证 IR 生成..." -ForegroundColor Magenta
$allPassed = (Run-Test "IR验证" "cargo test validate_llvm_ir --lib 2>&1") -and $allPassed
Write-Host ""

Write-Host "========================================" -ForegroundColor Cyan
if ($allPassed) {
    Write-Host "  所有测试通过!" -ForegroundColor Green
    $exitCode = 0
} else {
    Write-Host "  部分测试失败!" -ForegroundColor Red
    $exitCode = 1
}

$totalTime = (Get-Date) - $startTime
Write-Host " 总耗时: $([math]::Round($totalTime.TotalSeconds, 2))s" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

exit $exitCode