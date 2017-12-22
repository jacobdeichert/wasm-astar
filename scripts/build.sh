echo "==========================================================="
echo "BUILDING TO WASM"
echo "==========================================================="

THIS_SCRIPTS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd $THIS_SCRIPTS_DIR/..

wasmFilename="wasm_astar.wasm"

existingWasmFile="dist/$wasmFilename"
[ -e $existingWasmFile ] && rm $existingWasmFile

# build to wasm target and move to dist directory
cargo +nightly build --target wasm32-unknown-unknown --release \
    && mv "target/wasm32-unknown-unknown/release/$wasmFilename" dist


