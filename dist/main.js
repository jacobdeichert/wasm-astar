
const randomRange = (min, max) => {
  return Math.floor(Math.random() * (max + 1 - min)) + min;
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

class Transform {
  constructor(posX, posY, scaleX, scaleY) {
    this.position = { x: posX, y: posY };
    this.scale = { x: scaleX, y: scaleY };
  }
}

class Rect {
  constructor(x, y, width, height) {
    this.transform = new Transform(x, y, width, height);
    this.color = Color.random();
  }

  draw(ctx) {
    const { color, transform: { position, scale } } = this;
    const { h, s, l, a } = color;
    ctx.fillStyle = `hsla(${h}, ${s}%, ${l}%, ${a})`;
    ctx.fillRect(position.x, position.y, scale.x, scale.y);
  }
}

class GameEngine {
  constructor() {
    this.bindMethods(this);
    this.gridWidth = 900;
    this.gridHeight = 600;
    this.tileSize = 50;
    this.tiles = [];
    this.startTile = null;
    this.endTile = null;
    this.wasmModule = null;

    this.loadWasm().then(() => {
      this.initCanvas();
      this.generateTiles();
      this.setUpTargetTiles();
      this.tick();
    });
  }

  bindMethods(t) {
    const methods = [
      'tick',
      'draw',
      'update',
      'clearScreen',
      'loadWasm',
      'getTileAt',
      'generateTiles',
      'setUpTargetTiles',
      'initCanvas',
    ];
    methods.forEach(m => {
      t[m] = t[m].bind(t);
    });
  }

  loadWasm() {
    const { clearScreen, update, draw } = this;
    const wasmImports = {
      js_clear_screen: clearScreen,
      js_update: update,
      js_draw: draw,
    };
    return fetch('wasm_astar.wasm')
      .then(response => response.arrayBuffer())
      .then(bytes => WebAssembly.instantiate(bytes, { env: wasmImports }))
      .then(results => {
        const mod = results.instance;
        this.wasmModule = mod.exports;
        console.log(mod);
      });
  }

  clearScreen() {
    const { ctx, canvas } = this;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }

  initCanvas() {
    this.canvas = document.getElementById('canvas1');
    this.canvas.width = this.gridWidth;
    this.canvas.height = this.gridHeight;
    this.ctx = this.canvas.getContext('2d');
  }

  generateTiles() {
    const { tiles, gridWidth, gridHeight, tileSize } = this;

    for (let y = 0; y < gridHeight / tileSize; y++) {
      for (let x = 0; x < gridWidth / tileSize; x++) {
        const tile = new Rect(x * tileSize, y * tileSize, tileSize, tileSize);
        // Every other tile is true and rows are offset by one. This creates a checkerboard
        const checkerboardTest = (x + y) % 2 === 0;
        const lightness = checkerboardTest ? 10 : 20;
        tile.color = new Color(0, 0, lightness);
        tiles.push(tile);
      }
    }
  }

  getTileAt(x, y) {
    const { tiles, gridWidth, gridHeight, tileSize } = this;
    const numXTiles = gridWidth / tileSize;
    return tiles[x * numXTiles + y];
  }

  setUpTargetTiles() {
    const { tiles, getTileAt } = this;
    this.startTile = getTileAt(0, 0);
    this.endTile = getTileAt(8, 12);
    this.startTile.color = new Color(220, 100, 60);
    this.endTile.color = new Color(280, 100, 60);
  }

  update() {
    // for minimal neccessary client updates
  }

  draw() {
    const { ctx, tiles } = this;
    for (let i = 0; i < tiles.length; i++) {
      tiles[i].draw(ctx);
    }
  }

  tick() {
    const { wasmModule, tick } = this;
    this.wasmModule.tick();
    window.requestAnimationFrame(tick);
  }
}

window.addEventListener('load', () => new GameEngine());
