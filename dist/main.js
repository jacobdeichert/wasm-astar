
const WASM_ASTAR = {
  wasmModulePath: 'wasm_astar.wasm',
  world: null,
  debug: true,
  debugRenderIntervalMs: 5000, // Used in debug mode
};

class Color {
  constructor(h, s, l, a = 1) {
    this.h = h;
    this.s = s;
    this.l = l;
    this.a = a;
  }

  static random() {
    return new Color(
      randomRange(0, 360),
      randomRange(50, 100),
      randomRange(30, 80),
      1
    );
  }
}

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

  drawRect(px, py, sx, sy, color) {
    const { ctx } = this;
    const { h, s, l, a } = color;
    ctx.fillStyle = `hsla(${h}, ${s}%, ${l}%, ${a})`;
    ctx.fillRect(px, py, sx, sy);
  }
}

class World {
  constructor() {
    this.bindMethods(this);
    const width = 900;
    const height = 600;
    this.interval = WASM_ASTAR.debug ? WASM_ASTAR.debugRenderIntervalMs : false;
    this.wasmModuleTick = () => {};
    // Could have multiple canvas renderers (background, foreground) and render
    // at different frequencies
    this.renderer = new CanvasRenderer('main', width, height);
  }

  bindMethods(t) {
    t.setWasmModuleTicker = t.setWasmModuleTicker.bind(t);
    t.clearScreen = t.clearScreen.bind(t);
    t.update = t.update.bind(t);
    t.tick = t.tick.bind(t);
    t.nextTick = t.nextTick.bind(t);
    t.startTick = t.startTick.bind(t);
    t.drawTile = t.drawTile.bind(t);
  }

  setWasmModuleTicker(wasmModuleTicker) {
    this.wasmModuleTick = wasmModuleTicker;
  }

  clearScreen() {
    this.renderer.clearScreen();
  }

  update() {
    // for minimal neccessary client updates
  }

  tick() {
    console.log('tick');
    this.wasmModuleTick();
    this.nextTick();
  }

  nextTick() {
    if (this.interval) return;
    window.requestAnimationFrame(this.tick);
  }

  startTick() {
    this.tick();
    if (this.interval) {
      setInterval(() => {
        this.tick();
      }, this.interval);
    } else {
      this.nextTick();
    }
  }

  // TODO: should just export drawRect instead? More generic?
  // Would need a ctx id map for rust to send to the draw call.
  drawTile(px, py, size, colorH, colorS, colorL, colorA) {
    const c = new Color(colorH, colorS, colorL, colorA);
    this.renderer.drawRect(px, py, size, size, c);
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

const init = () => {
  WASM_ASTAR.world = new World();
  const wasmImports = {
    js_clear_screen: WASM_ASTAR.world.clearScreen,
    js_update: WASM_ASTAR.world.update,
    js_draw_tile: WASM_ASTAR.world.drawTile,
  };
  return loadWasm(WASM_ASTAR.wasmModulePath, wasmImports).then(wasmModule => {
    WASM_ASTAR.world.setWasmModuleTicker(wasmModule.tick);
    WASM_ASTAR.world.startTick();
  });
};

window.addEventListener('load', () => init());
