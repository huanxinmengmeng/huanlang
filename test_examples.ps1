# 运行 examples 目录下的所有 .hl 文件
$files = @(
    "algorithms.hl",
    "array_test.hl",
    "collections.hl",
    "control_flow.hl",
    "demo.hl",
    "error_handling.hl",
    "fibonacci.hl",
    "file_processing.hl",
    "functions.hl",
    "hello_world.hl",
    "import_selective.hl",
    "import_test.hl",
    "interop_demo.hl",
    "network_example.hl",
    "oop.hl",
    "simple_demo.hl",
    "sorting_algorithms.hl",
    "test_arr_get.hl",
    "test_bubble_sort.hl",
    "test_complete.hl",
    "test_run.hl",
    "test_var.hl",
    "transpile_demo.hl",
    "variables.hl",
    "综合示例.hl"
)

$total = $files.Count
$passed = 0
$failed = 0
$failedFiles = @()

Write-Host "=== 运行 examples 目录下的所有代码 ==="
Write-Host "总共 $total 个文件"
Write-Host ""

foreach ($file in $files) {
    Write-Host "运行: $file"
    $output = & "F:\huanlang\target\release\huan.exe" run "F:\huanlang\examples\$file" 2>&1
    $errorCount = ($output | Where-Object { $_ -like "*错误*" }).Count
    if ($errorCount -eq 0) {
        Write-Host "  ✓ 成功"
        $passed++
    } else {
        Write-Host "  ✗ 失败"
        $failed++
        $failedFiles += $file
    }
    Write-Host ""
}

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