# [WIP] Proper Python Embedding in Rust

This is part of strategy to bring Python packages into Python. The ultimate goal is to make any Python library available in Rust through Cargo.toml

```toml
[package]
name = "diffusers"
version = "0.26"

[dependencies]
python = "3.10" # This is the embedding

[build-dependencies]
pip = "24" # To download and setup the pip packages in [package.metadata.pip]

[package.metadata.pip]
diffusers = "0.26"
```

so anyone could use the generated library with a simple
```toml
[dependencies]
diffusers = "0.26"
```

without worrying about its runtime. In fact, the runtime would be entirely replaced with a Rust implementation and the user wouldn't know.

### Try the example

```bash
git clone https://github.com/sensorial-systems/python
cd python
cargo install --path cargo-python
cargo python run -p example
```
