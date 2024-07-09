@REM This file should be included in the stickdeck-win release zip.
@echo off

stickdeck-win.exe steamdeck 2>&1 | powershell "$input | tee log.txt"