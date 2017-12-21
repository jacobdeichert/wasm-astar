const WASM_ASTAR = {
  renderManager: null,
  wasmModulePath: 'wasm_astar.wasm',
  debug: true, // Wasm converts to an int
  renderIntervalMs: 10000, // Used in debug mode
};

const init = () => {
  const { wasmModulePath, debug, renderIntervalMs } = WASM_ASTAR;
  const renderManager = new RenderManager();
  WASM_ASTAR.renderManager = renderManager;
  const wasmImports = {
    js_random_range: randomRange,
    js_request_tick: renderManager.requestNextTick,
    js_start_interval_tick: renderManager.startIntervalTick,
    js_set_screen_size: renderManager.setScreenSize,
    js_set_renderer_size: renderManager.setRendererSize,
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
  constructor(canvasId) {
    this.bindMethods(this);
    this.canvas = document.createElement('canvas');
    this.canvas.id = canvasId;
    this.canvas = document.getElementById('renderer').appendChild(this.canvas);
    this.ctx = this.canvas.getContext('2d');
  }

  bindMethods(t) {
    t.setSize = t.setSize.bind(t);
    t.clearScreen = t.clearScreen.bind(t);
    t.drawRect = t.drawRect.bind(t);
    t.drawText = t.drawText.bind(t);
  }

  setSize(width, height, quality) {
    this.canvas.width = width;
    this.canvas.height = height;
    this.canvas.style.width = `${width / quality}px`;
    this.canvas.style.height = `${height / quality}px`;
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

  drawText(text, fontSize, px, py) {
    const { ctx } = this;
    ctx.fillStyle = '#fff';
    ctx.font = `${fontSize}px Monaco, Consolas, Courier, monospace`;
    ctx.fillText(text, px, py);
  }
}

class RenderManager {
  constructor() {
    this.bindMethods(this);
    this.isIntervalTick = false;
    this.wasmModuleTick = () => {};
    this.lastUpdateTime = 0;
    this.lastFpsRenderTime = 0;
    this.fps = 0;
    // Could have multiple canvas renderers (background, foreground) and render
    // at different frequencies. Their index is their id which rust/wasm passes
    // down to certain functions.
    this.renderers = [
      new CanvasRenderer('tile_bg'),
      new CanvasRenderer('main'),
      new CanvasRenderer('fps'),
    ];
  }

  bindMethods(t) {
    t.setScreenSize = t.setScreenSize.bind(t);
    t.setRendererSize = t.setRendererSize.bind(t);
    t.setWasmModuleTicker = t.setWasmModuleTicker.bind(t);
    t.clearScreen = t.clearScreen.bind(t);
    t.update = t.update.bind(t);
    t.requestNextTick = t.requestNextTick.bind(t);
    t.startIntervalTick = t.startIntervalTick.bind(t);
    t.drawTile = t.drawTile.bind(t);
  }

  setScreenSize(width, height, quality) {
    const wrapper = document.getElementById('renderer');
    wrapper.style.width = `${width / quality}px`;
    wrapper.style.height = `${height / quality}px`;
  }

  setRendererSize(rendererId, width, height, quality) {
    this.renderers[rendererId].setSize(width, height, quality);
  }

  setWasmModuleTicker(wasmModuleTicker) {
    this.wasmModuleTick = wasmModuleTicker;
  }

  clearScreen(rendererId) {
    this.renderers[rendererId].clearScreen();
  }

  // for minimal neccessary client updates
  update() {
    if (this.lastUpdateTime) {
      if (Math.floor(this.lastUpdateTime) % 9 === 0) {
        const delta = (performance.now() - this.lastUpdateTime) / 1000;
        this.fps = 1 / delta;
      }
    }
    this.lastUpdateTime = performance.now();
  }

  requestNextTick() {
    this.drawFps(); // TODO: move call to wasm
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

  drawFps() {
    const renderDelayMs = 50;
    if (this.lastFpsRenderTime > this.lastUpdateTime - renderDelayMs) return;
    this.lastFpsRenderTime = performance.now();
    this.clearScreen(2);
    this.renderers[2].drawText(`fps: ${Math.ceil(this.fps)}`, 40, 5, 45);
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
