@REM Run this in the `win` folder: `scripts\release.bat v0.1.0`
@echo off
if "%1" == "" goto error

cargo build --release
del stickdeck-win-%1.zip || echo.
PowerShell.exe -NoProfile -Command "Compress-Archive -Path 'target\release\stickdeck-win.exe' -DestinationPath 'stickdeck-win-%1.zip'"
gh release delete-asset %1 stickdeck-win-%1.zip -y || echo.
gh release upload %1 stickdeck-win-%1.zip
goto end

:error
echo Error: You must provide a version as a command-line argument.
exit /b 1

:end