# Let's get Rusty!
Repo for a Rust tech talk

## Getting Started

- Install [rustup](https://rustup.rs/) for your operating system - this will also install `cargo`, rust's package manager
- `cargo run` at the root will run whatever function is in `main.rs`

## Troubleshooting
If you're having issues after the `Cargo.toml` file has been updated, delete your lock file before re-installing all dependencies:
```
$ rm -r Cargo.lock 
$ cargo build
```