# Rust WebAssembly A* Pathfinding Demo


![demo gif](dist/demo.gif)


## Building

Follow the [instructions on Hello Rust](https://www.hellorust.com/setup/wasm-target/) to get the wasm toolchain set up. Below is a summary of the steps required.

~~~sh
# Install the latest nightly
rustup toolchain install nightly

# Add wasm as a target
rustup target add wasm32-unknown-unknown --toolchain nightly

# Install wasm-gc to shrink the output file (optional)
cargo install --git https://github.com/alexcrichton/wasm-gc

# Run the build script
./scripts/build.sh
~~~
