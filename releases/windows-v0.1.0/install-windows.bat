@echo off

rem Assuming your binary is named "your_cli_tool" and is in the same directory as this script
set BIN_NAME=mints
set DEST_DIR=%USERPROFILE%\bin

rem Create the destination directory if it doesn't exist
if not exist "%DEST_DIR%" mkdir "%DEST_DIR%"

rem Copy the binary to the destination directory
copy "%BIN_NAME%" "%DEST_DIR%"

echo Installation complete. %BIN_NAME% is now available in your PATH.
