const WASM_ASTAR = {
  wasmModule: null,
  wasmModulePath: 'wasm_astar',
  debug: false, // Wasm converts to an int
  renderIntervalMs: 1000, // Used in debug mode
  // Can have multiple canvas layers (background, foreground) and render
  // at different frequencies.
  layers: new Map(),
  layerWrapperEl: document.getElementById('layer_wrapper'),
};

const init = () => {
  const { wasmModulePath, debug, renderIntervalMs } = WASM_ASTAR;
  return loadWasm(wasmModulePath, getWasmImports()).then(wasmModule => {
    WASM_ASTAR.wasmModule = wasmModule;
    WASM_ASTAR.wasmModule.init(
      debug,
      renderIntervalMs,
      window.innerWidth,
      window.innerHeight
    );
    window.addEventListener('mousemove', e => {
      // Coordinates relative to the canvas layers. Top left is 0,0
      const x = e.pageX - WASM_ASTAR.layerWrapperEl.offsetLeft;
      const y = e.pageY - WASM_ASTAR.layerWrapperEl.offsetTop;
      WASM_ASTAR.wasmModule.mouse_move(x, y);
    });
    window.addEventListener('keydown', e => {
      WASM_ASTAR.wasmModule.key_down(e.keyCode);
    });
    window.addEventListener('keyup', e => {
      WASM_ASTAR.wasmModule.key_up(e.keyCode);
    });
    WASM_ASTAR.layerWrapperEl.addEventListener('touchend', e => {
      // Soooooo hacky. Sorry. Last minute mobile support.. simulating a spacebar
      // to cause map randomization.
      WASM_ASTAR.wasmModule.key_down(32);
      setTimeout(() => {
        WASM_ASTAR.wasmModule.key_up(32);
      }, 100);
    });
  });
};

const getWasmImports = () => {
  let isIntervalTick = false;

  // NOTE: i've prepended `js_` to each function name so it's very explicit
  // and easy to find where this interop layer is used on the rust side.
  return {
    // ========================================================================
    // SET UP ENGINE CALLS
    // ========================================================================
    js_random() {
      return Math.random();
    },

    js_random_range(min, max) {
      return Math.floor(Math.random() * (max + 1 - min)) + min;
    },

    js_log(ptr, length) {
      const msg = wasmReadStrFromMemory(ptr, length);
      console.log(`%cWASM >%c${msg}`, 'color: #ae59ff;');
    },

    js_request_tick() {
      if (isIntervalTick) return;
      window.requestAnimationFrame(WASM_ASTAR.wasmModule.tick);
    },

    js_start_interval_tick(ms) {
      console.log(`start interval tick`);
      isIntervalTick = true;
      // If I immediately call wasmModule.tick, the rust WORLD_STATE mutex
      // doesn't get unlocked and throws an error. So instead, we do an
      // immediate setTimeout so it occurs on the next stack frame.
      setTimeout(() => {
        return WASM_ASTAR.wasmModule.tick(performance.now());
      }, 0);
      setInterval(() => {
        return WASM_ASTAR.wasmModule.tick(performance.now());
      }, ms);
    },

    js_create_layer(idPtr, idLength, key) {
      const canvas = document
        .getElementById('layer_wrapper')
        .appendChild(document.createElement('canvas'));
      canvas.id = wasmReadStrFromMemory(idPtr, idLength);
      const ctx = canvas.getContext('2d');

      // Note: key is an int for easy passing between wasm/js
      WASM_ASTAR.layers.set(key, {
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
        drawCircle(px, py, r, ch, cs, cl, ca) {
          ctx.fillStyle = `hsla(${ch}, ${cs}%, ${cl}%, ${ca})`;
          ctx.beginPath();
          ctx.arc(px, py, r, 0, Math.PI * 2, true);
          ctx.closePath();
          ctx.fill();
        },
        drawText(text, fontSize, px, py) {
          ctx.fillStyle = '#fff';
          ctx.font = `${fontSize}px Monaco, Consolas, Courier, monospace`;
          ctx.fillText(text, px, py);
        },
      });
    },

    js_set_screen_size(width, height, quality) {
      const wrapper = document.getElementById('layer_wrapper');
      wrapper.style.width = `${width / quality}px`;
      wrapper.style.height = `${height / quality}px`;
    },

    js_set_layer_size(layerId, width, height, quality) {
      WASM_ASTAR.layers.get(layerId).setSize(width, height, quality);
    },

    js_clear_screen(layerId) {
      WASM_ASTAR.layers.get(layerId).clearScreen();
    },

    // ========================================================================
    // SET UP DRAW CALLS
    // ========================================================================

    js_update() {
      // for minimal neccessary client updates
    },

    js_draw_tile(layerId, px, py, size, ch, cs, cl, ca) {
      WASM_ASTAR.layers
        .get(layerId)
        .drawRect(px, py, size, size, ch, cs, cl, ca);
    },

    js_draw_circle(layerId, px, py, r, ch, cs, cl, ca) {
      WASM_ASTAR.layers.get(layerId).drawCircle(px, py, r, ch, cs, cl, ca);
    },

    js_draw_fps(layerId, fps) {
      WASM_ASTAR.layers
        .get(layerId)
        .drawText(`fps: ${Math.round(fps)}`, 35, 5, 45);
    },

    js_path_count(layerId, count) {
      WASM_ASTAR.layers.get(layerId).drawText(`path: ${count}`, 35, 5, 95);
    },
  };
};

// Learned this from a blog post: Getting started with Rust/WebAssembly
// https://maffydub.wordpress.com/2017/12/02/getting-started-with-rust-webassembly/
// QUESTION: are there any issues with this method? Alternative/faster solutions?
const wasmReadStrFromMemory = (ptr, length) => {
  const buf = new Uint8Array(WASM_ASTAR.wasmModule.memory.buffer, ptr, length);
  return new TextDecoder('utf8').decode(buf);
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
