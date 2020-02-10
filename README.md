![Rust](https://github.com/dgunay/rpg-dice-rust/workflows/Rust/badge.svg)

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

## Building for Android/Termux

You can cross-compile the project to run on Termux. Download the Android Native 
Development Kit (NDK) and the appropriate Rust target, and run the command
like this:

```sh
$ env CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=/path/to/ndk/android-ndk-r21/toolc
hains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android29-clang cargo build --target aarch64-linux-android
```

That example is for my phone (Galaxy Note 8), you may need a different Clang
toolchain and target depending on your phone's CPU architecture.