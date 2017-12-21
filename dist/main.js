const WASM_ASTAR = {
  wasmModule: null,
  wasmModulePath: 'wasm_astar.wasm',
  debug: true, // Wasm converts to an int
  renderIntervalMs: 10000, // Used in debug mode

  // Can have multiple canvas layers (background, foreground) and render
  // at different frequencies. Their index is their id which rust/wasm passes
  // down to certain functions.
  layers: [],
};

const init = () => {
  const { wasmModulePath, debug, renderIntervalMs } = WASM_ASTAR;
  WASM_ASTAR.layers = [
    createLayer('tile_bg'),
    createLayer('main'),
    createLayer('fps'),
  ];
  return loadWasm(wasmModulePath, getWasmImports()).then(wasmModule => {
    WASM_ASTAR.wasmModule = wasmModule;
    WASM_ASTAR.wasmModule.init(debug, renderIntervalMs);
  });
};

const getWasmImports = () => {
  let isIntervalTick = false;
  let lastUpdateTime = 0;
  let lastFpsRenderTime = 0;
  let fps = 0;

  return {
    js_random_range(min, max) {
      return Math.floor(Math.random() * (max + 1 - min)) + min;
    },

    js_request_tick() {
      const renderDelayMs = 150;
      if (lastFpsRenderTime + renderDelayMs < lastUpdateTime) {
        lastFpsRenderTime = performance.now();
        WASM_ASTAR.layers[2].clearScreen();
        WASM_ASTAR.layers[2].drawText(`fps: ${Math.round(fps)}`, 40, 5, 45);
      }

      if (isIntervalTick) return;
      window.requestAnimationFrame(WASM_ASTAR.wasmModule.tick);
    },

    js_start_interval_tick(ms) {
      console.log(`start interval tick`);
      isIntervalTick = true;
      // If I immediately call wasmModule.tick, the rust WORLD_STATE mutex
      // doesn't get unlocked and throws an error.
      // So instead, we do an immediate setTimeout so it occurs
      // on the next stack frame.
      setTimeout(WASM_ASTAR.wasmModule.tick, 0);
      setInterval(WASM_ASTAR.wasmModule.tick, ms);
    },

    js_set_screen_size(width, height, quality) {
      const wrapper = document.getElementById('renderer');
      wrapper.style.width = `${width / quality}px`;
      wrapper.style.height = `${height / quality}px`;
    },

    js_set_renderer_size(layerId, width, height, quality) {
      WASM_ASTAR.layers[layerId].setSize(width, height, quality);
    },

    js_clear_screen(layerId) {
      WASM_ASTAR.layers[layerId].clearScreen();
    },

    // for minimal neccessary client updates
    js_update() {
      if (lastUpdateTime) {
        const delta = (performance.now() - lastUpdateTime) / 1000;
        fps = 1 / delta;
      }
      lastUpdateTime = performance.now();
    },

    // ========================================================================
    // DRAW CALLS
    // ========================================================================
    js_draw_tile(layerId, px, py, size, ch, cs, cl, ca) {
      WASM_ASTAR.layers[layerId].drawRect(px, py, size, size, ch, cs, cl, ca);
    },
  };
};

const createLayer = id => {
  const canvas = document
    .getElementById('renderer')
    .appendChild(document.createElement('canvas'));
  canvas.id = id;
  const ctx = canvas.getContext('2d');

  return {
    ctx,
    canvas,
    setSize(width, height, quality) {
      canvas.width = width;
      canvas.height = height;
      canvas.style.width = `${width / quality}px`;
      canvas.style.height = `${height / quality}px`;
    },
    clearScreen() {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
    },
    drawRect(px, py, sx, sy, ch, cs, cl, ca) {
      ctx.fillStyle = `hsla(${ch}, ${cs}%, ${cl}%, ${ca})`;
      ctx.fillRect(px, py, sx, sy);
    },
    drawText(text, fontSize, px, py) {
      ctx.fillStyle = '#fff';
      ctx.font = `${fontSize}px Monaco, Consolas, Courier, monospace`;
      ctx.fillText(text, px, py);
    },
  };
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
