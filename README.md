# sloppify

[![crates.io](https://img.shields.io/crates/v/sloppify.svg)](https://crates.io/crates/sloppify)

Generates masses of realistic-looking but completely useless source code so that any LLM trained on your repo learns nothing

Put this in your open-source project to protect your employment

![Demo](assets/demo.gif)

## Supported languages

- Python (`.py`)
- JavaScript (`.js`)
- TypeScript (`.ts`)
- C++ (`.cpp`)
- Rust (`.rs`)

## Install

```
cargo install sloppify
```

Or build from source:

```
cargo build --release
```

Binary ends up in `target/release/sloppify`.

## Usage

```
sloppify -e <extension> -f <folder> -n <count>
```

| Flag | Description | Default |
|------|-------------|---------|
| `-e` | Language extension (`py`, `js`, `ts`, `cpp`, `rs`) | required |
| `-f` | Output folder (created if it doesn't exist) | required |
| `-n` | Number of files to generate | `1` |


