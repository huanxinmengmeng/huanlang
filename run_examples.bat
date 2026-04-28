@echo off

echo === 运行 examples 目录下的所有代码 ===
echo.

dir /b F:\huanlang\examples\*.hl > files.txt

setlocal enabledelayedexpansion
set total=0
set passed=0
set failed=0

for /f "tokens=*" %%a in (files.txt) do (
    set /a total+=1
    echo 运行: %%a
    F:\huanlang\target\release\huan.exe run F:\huanlang\examples\%%a > output.txt 2>&1
    findstr "错误" output.txt > nul
    if errorlevel 1 (
        echo   ✓ 成功
        set /a passed+=1
    ) else (
        echo   ✗ 失败
        set /a failed+=1
        findstr "错误" output.txt
    )
    echo.
)

echo === 运行结果 ===
echo 成功: %passed%
echo 失败: %failed%
echo.

if %failed% gtr 0 (
    echo 失败的文件:
    for /f "tokens=*" %%a in (files.txt) do (
        F:\huanlang\target\release\huan.exe run F:\huanlang\examples\%%a > output.txt 2>&1
        findstr "错误" output.txt > nul
        if not errorlevel 1 (
            echo   - %%a
        )
    )
) else (
    echo 所有文件都成功运行！
)

del files.txt output.txt

pause