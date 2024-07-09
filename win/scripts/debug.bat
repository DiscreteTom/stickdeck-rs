@REM This file should be included in the stickdeck-win release zip.
@echo off & setlocal

cmd /c "set RUST_LOG=debug && launch.bat"
pause
