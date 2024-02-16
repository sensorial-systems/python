# Proper Python Embedding in Rust

This is a PoC for embedding Python runtime in Rust libraries.

### Goal

The goal is to make it possible to implement crates that imports and redistributes pip modules like in [examples/ensta](examples/ensta) to be used like in [examples/example](examples/example) **seamlessly so the user wouldn't know that the `ensta` crate is actually implemented in Python**.

### ensta

examples/ensta/Cargo.toml
```toml
[package]
name = "ensta"
version = "0.1.0"
edition = "2021"

[dependencies]
pyo3 = { version = "0.20", features = ["auto-initialize"] }
python310 = { path = "../../python310" }

[build-dependencies]
python310 = { path = "../../python310" }

[package.metadata.pip]
ensta = "*"
```

examples/example/Cargo.toml
```toml
[package]
name = "example"
version = "0.1.0"
edition = "2021"

[dependencies]
ensta = { path = "../ensta" }
```

examples/example/src/main.rs
```rust
use ensta::Client;

fn main() {
    let client = Client::new();
    println!("{}", client.biography_of("notdanilo"));
}
```

### Try the example

```bash
git clone https://github.com/sensorial-systems/python
cd python
cargo install --path cargo-python
cargo python run -p example
```
