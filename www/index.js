import { Universe, Cell, Pattern } from "game-of-life";
// Import the WebAssembly memory
import { memory } from "game-of-life/game_of_life_bg";

// constants for cell pixel size and cell colors
const CELL_SIZE = 6;
const CELL_BORDER = CELL_SIZE + 1;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe with a given width and height
const width = 88;
const height = 88;
let universe = Universe.new_rand(width, height);
let pattern = null;

const playPauseButton = document.getElementById("play-pause");
const stepButton = document.getElementById("step");
const resetButton = document.getElementById("reset");
const clearButton = document.getElementById("clear");
const patternSelect = document.getElementById("pattern");
// Give the canvas room for the cells and a 1px border around each
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * height + 1;

const ctx = canvas.getContext('2d');
let animationId = null;

// Render loop, runs each frame
const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};

// Methods for drawing the grid and cells to the canvas
// ----------------------------------------------------
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
};

const getIndex = (row, column) => {
    return row * width + column;
};

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
};
// ----------------------------------------------------

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};

// Methods for play and pause functionality
// ----------------------------------------
const isPaused = () => {
    return animationId === null;
};

const play = () => {
    playPauseButton.textContent = "⏸︎";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};
// ----------------------------------------

// Event listeners for buttons and canvas
// --------------------------------------
playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

// Event listener for step button
stepButton.addEventListener("click", event => {
    if (isPaused()) {
        universe.tick();

        drawGrid();
        drawCells();
    }
})

// Event listener for reset button
resetButton.addEventListener("click", event => {
    universe = Universe.new_rand(width, height);

    // Redraw the scene, in case we're currently paused
    drawGrid();
    drawCells();
})

// Event listener for clear button
clearButton.addEventListener("click", event => {
    universe = Universe.new(width, height);

    // Redraw the scene, in case we're currently paused
    drawGrid();
    drawCells();
})

patternSelect.addEventListener("change", event => {
    if (patternSelect.value == "cell") {
        pattern = null;
    } 
    else if (patternSelect.value == "blinker") {
        pattern = Pattern.blinker();
    }
    else if (patternSelect.value == "toad") {
        pattern = Pattern.toad();
    }
    else if (patternSelect.value == "beacon") {
        pattern = Pattern.beacon();
    }
    else if (patternSelect.value == "pulsar") {
        pattern = Pattern.pulsar();
    }
    else if (patternSelect.value == "glider") {
        pattern = Pattern.glider();
    }
    else if (patternSelect.value == "lwss") {
        pattern = Pattern.lightweight_spaceship();
    }
    else if (patternSelect.value == "mwss") {
        pattern = Pattern.midweight_spaceship();
    }
    else if (patternSelect.value == "hwss") {
        pattern = Pattern.heavyweight_spaceship();
    }
});

// Event listener for canvas, to toggle cells
canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();
    // Convert the page relative click coordinates to canvas relative
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;
    // Get the row and column from the canvas relative coordinates
    const row = Math.min(Math.floor(canvasTop / CELL_BORDER), height - 1);
    const col = Math.min(Math.floor(canvasLeft / CELL_BORDER), height - 1);

    if (pattern == null){
        universe.toggle_cell(row, col);
    } else {
        universe.insert_pattern(pattern, row, col);
    }

    // Redraw the scene (most likely we will be toggling cells when the game is paused,
    // so they wouldn't be redrawn until the game was running again otherwise)
    drawGrid();
    drawCells();
});

// --------------------------------------

play();