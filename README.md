# Klayer
A cli based music player

## Prerequisites

### Clone the source code

First clone the source code:

```sh
git clone https://github.com/prajnastra/klayer.git
cd klayer 
```

### Install the Rust compiler with `rustup`

1. Install [`rustup.rs`](https://rustup.rs/).

3. To make sure you have the right Rust compiler installed, run

   ```sh
   rustup override set stable
   rustup update stable
   ```

## Building

### Linux 

```sh
cargo build --release
```

### Install
```sh
mkdir -p ~/.local/bin
cp target/release/klayer ~/.local/bin/
```

## Usage

### See available args

```bash
klayer -h
```

### Play music

```bash
klayer -p hello.mp3
```
