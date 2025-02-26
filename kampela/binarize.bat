@echo off
setlocal enabledelayedexpansion

echo Building...

cargo build --release

set BUILD_DIRECTORY=target\thumbv8m.main-none-eabihf
set EXECUTABLE=%BUILD_DIRECTORY%\release\app
set FIRMWARE=%BUILD_DIRECTORY%\release\app.bin

echo Preparing binary file...

arm-none-eabi-objcopy.exe -O binary "%EXECUTABLE%" "%FIRMWARE%"

echo Flashing...
if "%1"=="" (
    pilkki.exe write -i "%FIRMWARE%"
) else (
    pilkki.exe --port %1 write -i "%FIRMWARE%"
)

