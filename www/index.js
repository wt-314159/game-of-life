import { Universe, Cell } from "game-of-life";
// Import the WebAssembly memory
import { memory } from "game-of-life/game_of_life_bg";

// constants for cell pixel size and cell colors
const CELL_SIZE = 6;
const CELL_BORDER = CELL_SIZE + 1;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe with a given width and height
const width = 100;
const height = 100;
const universe = Universe.new_rand(width, height);

// Give the canvas room for the cells and a 1px border around each
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * height + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    debugger;
    drawGrid();
    drawCells();

    universe.tick();

    requestAnimationFrame(renderLoop);
}

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * CELL_BORDER + 1, 0);
        ctx.lineTo(i * CELL_BORDER + 1, CELL_BORDER * height + 1);
    }

    // Horizontal lines
    for (let j = 0; j<= height; j++) {
        ctx.moveTo(0, j * CELL_BORDER + 1);
        ctx.lineTo(width * CELL_BORDER + 1, j * CELL_BORDER + 1);
    }

    ctx.stroke();
}

const getIndex = (row, column) => {
    return row * width + column;
}

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = bitIsSet(idx, cells) 
                ? ALIVE_COLOR
                : DEAD_COLOR;
            
            ctx.fillRect(
                col * CELL_BORDER + 1,
                row * CELL_BORDER + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
}

requestAnimationFrame(renderLoop);