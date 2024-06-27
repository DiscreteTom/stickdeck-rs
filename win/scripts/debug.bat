@REM This file should be included in the stickdeck-win release zip.
@echo off

setlocal
  set RUST_LOG=debug && stickdeck-win.exe
endlocal

pause