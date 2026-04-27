param(
    [string]$InputFile,
    [switch]$Clean
)

if ($Clean) {
    Write-Host "Cleaning build artifacts..." -ForegroundColor Cyan
    Remove-Item -Path "*.ll" -ErrorAction SilentlyContinue
    Remove-Item -Path "*.s" -ErrorAction SilentlyContinue
    Remove-Item -Path "*.exe" -ErrorAction SilentlyContinue
    Write-Host "Cleanup completed" -ForegroundColor Green
    exit 0
}

if (-not $InputFile) {
    Write-Host "Usage: build.ps1 -InputFile <file>" -ForegroundColor Yellow
    Write-Host "Example: build.ps1 -InputFile test.hl" -ForegroundColor Yellow
    Write-Host "Options:" -ForegroundColor Yellow
    Write-Host "  -Clean    Clean build artifacts" -ForegroundColor Yellow
    exit 1
}

if (-not (Test-Path $InputFile)) {
    Write-Host "Error: File not found: $InputFile" -ForegroundColor Red
    exit 1
}

$baseName = [System.IO.Path]::GetFileNameWithoutExtension($InputFile)
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "HuanLang 编译器" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "输入文件: $InputFile" -ForegroundColor White
Write-Host "输出文件: $baseName.exe" -ForegroundColor White
Write-Host ""

$env:CARGO_INCREMENTAL = "0"

if (-not (Test-Path "target\release\huan.exe")) {
    Write-Host "[1/4] 编译 HuanLang 编译器..." -ForegroundColor Yellow
    cargo build --release 2>&1 | Select-Object -Last 5
    if ($LASTEXITCODE -ne 0) {
        Write-Host "编译器构建失败!" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "[1/4] 编译器已就绪" -ForegroundColor Green
}

Write-Host "[2/4] 生成 LLVM IR..." -ForegroundColor Yellow
target\release\huan.exe build $InputFile 2>&1 | Out-Host

if (-not (Test-Path "$baseName.ll")) {
    Write-Host "LLVM IR 生成失败!" -ForegroundColor Red
    exit 1
}

if (Test-Path "$baseName.exe") {
    Write-Host "[3/4] 可执行文件已生成" -ForegroundColor Green
} else {
    Write-Host "[3/4] 可执行文件生成失败" -ForegroundColor Red
    Write-Host "LLVM IR 文件位于: $baseName.ll" -ForegroundColor Yellow
    exit 1
}

Write-Host "[4/4] 运行测试..." -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
& ".\$baseName.exe"
$exitCode = $LASTEXITCODE
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "程序退出码: $exitCode" -ForegroundColor Cyan

if ($exitCode -eq 0) {
    Write-Host "执行成功!" -ForegroundColor Green
} else {
    Write-Host "程序返回非零退出码: $exitCode" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "构建完成!" -ForegroundColor Green
exit $exitCode
