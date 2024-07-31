# Leet
Simple 1337 CLI generator

## Installation

### Compilation

```shell
cargo build --release
```

### Copy somewhere in your PATH

For instance
```shell
cp target/release/leet $HOME/.local/bin
```

## Usage

```shell
echo 'Whatever you want' | leet
```

Multilines STDOUT can also be pipped in **leet**

```shell
cat Cargo.toml | leet
```
