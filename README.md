# solc-align

A tool to align solidity contract storage and calldata struct to 256 bits to maximizing storage reduction.

## Features

- [x] Align to 32 bytes(256 bits) for calldata struct.
- [x] Use stable sort to make the same size bytes type in the same order as input.
- [ ] Align contract storage, which needs AST parser.

## Install

```sh
cargo install --git https://github.com/Akagi201/solc-align.git
```

## Usage

read input.txt for example.

case 1:

just print to stdout.

```sh
solc-align ./input.txt
```

case 2:

write result to a file.

```sh
solc-align ./input.txt ./output.txt
```
