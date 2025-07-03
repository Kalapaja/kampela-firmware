@echo off
setlocal enabledelayedexpansion

echo Flashing eflash with wordlist...

set SUBCOMMAND_NEEDED="write-eflash"

pilkki.exe --help | findstr /i "%SUBCOMMAND_NEEDED%">nul || (
    echo Your version of pilkki doesn't support this feature, try to find newer version
    exit /b -1
) 

set WORDLIST=".\wordlist.bin"
set BASE_ADDRESS=0x00008000
if "%1"=="" (
	pilkki write-eflash -i "%WORDLIST%" -a %BASE_ADDRESS%
) else (
	pilkki --port %1 write-eflash -i "%WORDLIST%" -a %BASE_ADDRESS%
)

exit /b -1