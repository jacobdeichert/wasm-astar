// @ts-check

const WASM_ASTAR = {
  wasmModulePath: 'wasm_astar.wasm',
  renderManager: null,
  debug: true,
  renderIntervalMs: 10000, // Used in debug mode
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
  constructor() {
    this.bindMethods(this);
    const width = 900;
    const height = 600;
    this.isIntervalTick = false;
    this.wasmModuleTick = () => {};
    // Could have multiple canvas renderers (background, foreground) and render
    // at different frequencies. Their index is their id which rust/wasm passes
    // down to certain functions.
    this.renderers = [new CanvasRenderer('main', width, height)];
    this.renderersByName = {
      main: this.renderers[0],
    };
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
    this.wasmModuleTick();
    setInterval(this.wasmModuleTick, ms);
  }

  // TODO: should just export drawRect instead? More generic?
  // Would need a ctx id map for rust to send to the draw call.
  drawTile(px, py, size, ch, cs, cl, ca) {
    this.renderersByName.main.drawRect(px, py, size, size, ch, cs, cl, ca);
  }
}

const loadWasm = (filepath, wasmImports) => {
  return fetch(filepath)
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, { env: wasmImports }))
    .then(results => {
      return results.instance.exports;
    });
};

const init = () => {
  WASM_ASTAR.renderManager = new RenderManager();
  const wasmImports = {
    js_request_tick: WASM_ASTAR.renderManager.requestNextTick,
    js_start_interval_tick: WASM_ASTAR.renderManager.startIntervalTick,
    js_clear_screen: WASM_ASTAR.renderManager.clearScreen,
    js_update: WASM_ASTAR.renderManager.update,
    js_draw_tile: WASM_ASTAR.renderManager.drawTile,
  };
  return loadWasm(WASM_ASTAR.wasmModulePath, wasmImports).then(wasmModule => {
    WASM_ASTAR.renderManager.setWasmModuleTicker(wasmModule.tick);
    const debug = WASM_ASTAR.debug ? 1 : 0;
    wasmModule.init(debug, WASM_ASTAR.renderIntervalMs);
  });
};

window.addEventListener('load', () => init());
