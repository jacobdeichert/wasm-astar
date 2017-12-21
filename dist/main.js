const WASM_ASTAR = {
  renderManager: null,
  wasmModulePath: 'wasm_astar.wasm',
  width: 900,
  height: 600,
  debug: true, // Wasm converts to an int
  renderIntervalMs: 10000, // Used in debug mode
};

const init = () => {
  const { wasmModulePath, width, height, debug, renderIntervalMs } = WASM_ASTAR;
  const renderManager = new RenderManager(width, height);
  WASM_ASTAR.renderManager = renderManager;
  const wasmImports = {
    js_random_range: randomRange,
    js_request_tick: renderManager.requestNextTick,
    js_start_interval_tick: renderManager.startIntervalTick,
    js_clear_screen: renderManager.clearScreen,
    js_update: renderManager.update,
    js_draw_tile: renderManager.drawTile,
  };
  return loadWasm(wasmModulePath, wasmImports).then(wasmModule => {
    renderManager.setWasmModuleTicker(wasmModule.tick);
    wasmModule.init(debug, renderIntervalMs);
  });
};

class CanvasRenderer {
  constructor(canvasId, width, height) {
    this.bindMethods(this);
    this.canvas = document.getElementById(canvasId);
    this.canvas.width = width;
    this.canvas.height = height;
    this.ctx = this.canvas.getContext('2d');
  }

  bindMethods(t) {
    t.clearScreen = t.clearScreen.bind(t);
    t.drawRect = t.drawRect.bind(t);
  }

  clearScreen() {
    const { ctx, canvas } = this;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }

  drawRect(px, py, sx, sy, ch, cs, cl, ca) {
    const { ctx } = this;
    ctx.fillStyle = `hsla(${ch}, ${cs}%, ${cl}%, ${ca})`;
    ctx.fillRect(px, py, sx, sy);
  }
}

class RenderManager {
  constructor(width, height) {
    this.bindMethods(this);
    this.isIntervalTick = false;
    this.wasmModuleTick = () => {};
    // Could have multiple canvas renderers (background, foreground) and render
    // at different frequencies. Their index is their id which rust/wasm passes
    // down to certain functions.
    this.renderers = [
      new CanvasRenderer('tile_bg', width, height),
      new CanvasRenderer('main', width, height),
    ];
  }

  bindMethods(t) {
    t.setWasmModuleTicker = t.setWasmModuleTicker.bind(t);
    t.clearScreen = t.clearScreen.bind(t);
    t.update = t.update.bind(t);
    t.requestNextTick = t.requestNextTick.bind(t);
    t.startIntervalTick = t.startIntervalTick.bind(t);
    t.drawTile = t.drawTile.bind(t);
  }

  setWasmModuleTicker(wasmModuleTicker) {
    this.wasmModuleTick = wasmModuleTicker;
  }

  clearScreen(rendererId) {
    this.renderers[rendererId].clearScreen();
  }

  update() {
    // for minimal neccessary client updates
  }

  requestNextTick() {
    if (this.isIntervalTick) return;
    window.requestAnimationFrame(this.wasmModuleTick);
  }

  startIntervalTick(ms) {
    console.log(`start interval tick`);
    this.isIntervalTick = true;
    // If I immediately call wasmModuleTick, the rust WORLD_STATE mutex
    // doesn't get unlocked and throws an error.
    // So instead, we do an immediate setTimeout so it occurs
    // on the next stack frame.
    setTimeout(this.wasmModuleTick, 0);
    setInterval(this.wasmModuleTick, ms);
  }

  // TODO: should just export drawRect instead? More generic?
  drawTile(rendererId, px, py, size, ch, cs, cl, ca) {
    this.renderers[rendererId].drawRect(px, py, size, size, ch, cs, cl, ca);
  }
}

const randomRange = (min, max) => {
  return Math.floor(Math.random() * (max + 1 - min)) + min;
};

const loadWasm = (filepath, wasmImports) => {
  return fetch(filepath)
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, { env: wasmImports }))
    .then(results => {
      return results.instance.exports;
    });
};

window.addEventListener('load', init);
