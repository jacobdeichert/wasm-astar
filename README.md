# Rust WebAssembly A* Pathfinding Demo


![demo gif](dist/demo.gif)


## Building

Follow the instructions on Hello Rust to get the wasm toolchain set up: https://www.hellorust.com/setup/wasm-target/

~~~sh
# Install the latest nightly
rustup toolchain install nightly

# Add wasm as a target
rustup target add wasm32-unknown-unknown --toolchain nightly

# Run the build script
./scripts/build.sh
~~~
