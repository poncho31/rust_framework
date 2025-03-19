@echo off
setlocal enabledelayedexpansion

if "%1"=="" (
    echo Please specify 'install' or 'update'
    exit /b 1
)

set MODE=%1

:: Execute commands from install_plugin.json
for %%c in (python_env node_env vscode_extension) do (
    echo Executing commands for %%c in %MODE% mode
    for /f "tokens=*" %%i in ('powershell -Command "Get-Content install_plugin.json | ConvertFrom-Json | Select-Object -ExpandProperty cmd | Select-Object -ExpandProperty %MODE% | Select-Object -ExpandProperty %%c"') do (
        echo Executing: %%i
        %%i
        if %errorlevel% neq 0 (
            echo Error: Command failed: %%i
            exit /b %errorlevel%
        )
    )
)

echo %MODE% completed at %date% %time%
endlocal