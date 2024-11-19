# Compatibility

This version of Kampela firmware is compatible with Siltti v0.10

# Project maturity

This is an early proof of concept prototype, use for development and at your own risk only.

# Security note

Note that deprecated metadata shortening algorithm by Alzymologist Oy is implemented here, as opposed to one deployed in Polkadot ecosystem, thus no on-chain verification is actually happening. Migration is generally considered a trivial task by the community, we welcome PRs with corresponding patch (to all related repositories).

# Prerequisites

## Archlinux

### Install rustup and ARM toolchain:
```sh
[sudo] pacman -S rustup arm-none-eabi-gcc arm-none-eabi-binutils
rustup update
rustup default stable
```

## MacOs (tested on M1)

### Install rustup
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
rustup default stable
```

### Install ARM toolchain

#### brew
```sh
brew install --cask gcc-arm-embedded
```

#### manual installation
download and install suitable darwin GNU-ARM package from [ARM GNU website](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)


# Preparations

```sh
rustup target add thumbv8m.main-none-eabihf
cargo install flip-link
```


# Build

```sh
cargo build --release
```

## Flashing

### Pilkki

For Pilkki flasher look [here](https://github.com/Alzymologist/pilkki).

```sh
./binarize.sh --pilkki
```
### Segger

For Segger you need to install [simplicity-commander](https://www.silabs.com/developers/mcu-programming-options).
In case of ArchLinux there is [AUR package](https://aur.archlinux.org/packages/simplicity-commander).

```sh
./binarize.sh --segger
```



