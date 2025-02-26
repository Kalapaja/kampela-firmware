#!/bin/bash
set -e

echo "Flashing eflash with wordlist..."

SUBCOMMAND_NEEDED='write-eflash'
if ! [[ "$(pilkki --help)" =~ $SUBCOMMAND_NEEDED ]]; then
  echo "Your version of pilkki doesn't support this feature, try to find newer version"
  exit -1
fi

WORDLIST="./wordlist.bin"
BASE_ADDRESS=0x00008000
pilkki write-eflash -i "$WORDLIST" -a $BASE_ADDRESS
exit -1
fi