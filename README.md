# solidity-storage-align

A tool to align solidity storage and struct to 256 bits to maximizing storage reduction.

## Install

```sh
cargo install --git https://github.com/Akagi201/solidity-storage-align.git
```

## Usage

read input.txt for example.

case 1:

just print to stdout.

```sh
solidity-storage-align ./input.txt
```

case 2:

write result to a file.

```sh
solidity-storage-align ./input.txt ./output.txt
```
