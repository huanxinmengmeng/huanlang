# 运行 examples 目录下的所有 .hl 文件
$examples = Get-ChildItem -Path "F:\huanlang\examples" -Filter "*.hl" | Sort-Object Name

$total = $examples.Count
$passed = 0
$failed = 0
$failedFiles = @()

Write-Host "=== 运行 examples 目录下的所有代码 ==="
Write-Host "总共 $total 个文件"
Write-Host ""

foreach ($file in $examples) {
    $fileName = $file.Name
    $filePath = $file.FullName
    
    Write-Host "运行: $fileName"
    
    # 运行文件
    $output = & "F:\huanlang\target\release\huan.exe" run "$filePath" 2>&1
    
    # 检查输出
    $errorCount = ($output | Where-Object { $_ -like "*错误*" }).Count
    
    if ($errorCount -eq 0) {
        Write-Host "  ✓ 成功"
        $passed++
    } else {
        Write-Host "  ✗ 失败"
        $failed++
        $failedFiles += $fileName
        # 显示错误信息
        $output | Where-Object { $_ -like "*错误*" } | ForEach-Object { Write-Host "    $_" }
    }
    
    Write-Host ""
}

# 显示结果
Write-Host "=== 运行结果 ==="
Write-Host "成功: $passed"
Write-Host "失败: $failed"
Write-Host ""

if ($failed -gt 0) {
    Write-Host "失败的文件:"
    $failedFiles | ForEach-Object { Write-Host "  - $_" }
} else {
    Write-Host "所有文件都成功运行！"
}