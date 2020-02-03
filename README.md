# RPG Dice Rust

I used a command line dice roller as a way of learning how Perl's package
ecosystem worked, so I figured I'd do it again as a modest Rust project.

It uses Regex, SmallRng, and Evalexpr to do dice rolling. Due to the use of
Evalexpr, it might also support more than I had intended initially (just 
integer math).

## Testing

Run tests:

```rust
cargo test
```

## Fuzzing 

Uses [cargo fuzz](https://github.com/rust-fuzz/cargo-fuzz). Requires a nightly 
compiler.

See fuzzing targets:

```rust
cargo fuzz list
```

Run a fuzzing target:

```rust
rustup run nightly cargo fuzz run <target>
```