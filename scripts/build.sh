echo "==========================================================="
echo "BUILDING TO WASM"
echo "==========================================================="

set -e # If any command fails, script exits immediately

THIS_SCRIPTS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd $THIS_SCRIPTS_DIR/..

wasmFilename="wasm_astar"

existingWasmFile="dist/$wasmFilename.wasm"
[ -e $existingWasmFile ] && rm $existingWasmFile

# Compile to wasm
cargo +nightly build --target wasm32-unknown-unknown --release

# Move to dist
mv "target/wasm32-unknown-unknown/release/$wasmFilename.wasm" "dist/$wasmFilename"

# Minify wasm output
# Note: if wasm-gc becomes too slow for development, create a separate script for a production build
wasm-gc "dist/$wasmFilename" "dist/$wasmFilename"
