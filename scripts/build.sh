echo "==========================================================="
echo "BUILDING TO WASM"
echo "==========================================================="

set -e # If any command fails, script exits immediately

THIS_SCRIPTS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd $THIS_SCRIPTS_DIR/..

wasmFilename="wasm_astar.wasm"

existingWasmFile="dist/$wasmFilename"
[ -e $existingWasmFile ] && rm $existingWasmFile

# Compile to wasm
cargo +nightly build --target wasm32-unknown-unknown --release

# Move to dist
mv "target/wasm32-unknown-unknown/release/$wasmFilename" dist

# Minify wasm output
# Note: if wasm-gc becomes too slow for development, create a separate script for a production build
wasm-gc dist/wasm_astar.wasm dist/wasm_astar.wasm