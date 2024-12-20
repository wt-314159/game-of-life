import { Universe, Pattern } from "game-of-life";
// Import the WebAssembly memory
import { memory } from "game-of-life/game_of_life_bg";

// constants for cell pixel size and cell colors
let CELL_SIZE = 6;
let CELL_BORDER = CELL_SIZE + 1;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe with a given width and height
let width = 100;
let height = 100;
let universe = Universe.new_rand(width, height);
let pattern = null;
let showGrid = true;

// Get the controls by ID
const playPauseButton = document.getElementById("play-pause");
const stepButton = document.getElementById("step");
const resetButton = document.getElementById("reset");
const clearButton = document.getElementById("clear");
const gridButton = document.getElementById("grid");
const borderCheckbox = document.getElementById("cell_border");
const cellSizeSelect = document.getElementById("cell_size");
const patternSelect = document.getElementById("pattern");
const rotation = document.getElementById("rotation");
const canvas = document.getElementById("game-of-life-canvas");

const ctx = canvas.getContext('2d');
let animationId = null;

//                  FPS Class
// ================================================
const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames=[];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert delta time since last frame render into FPS
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the 100 latest timings
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find max, min, and mean of 100 latest timings
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(min, this.frames[i]);
            max = Math.max(max, this.frames[i]);
        }
        let mean = sum / this.frames.length;

        // Render statistics
        this.fps.textContent = `
        FPS:
         latest = ${Math.round(fps)}  
avg of last 100 = ${Math.round(mean)}  
min of last 100 = ${Math.round(min)}  
max of last 100 = ${Math.round(max)}`.trim();
    }
};
// ================================================

// Method to set cell size and cell border size
const setCellSize = () => {
    CELL_SIZE = parseInt(cellSizeSelect.value);
    if (borderCheckbox.checked) {
        CELL_BORDER = CELL_SIZE + 1;
    }
    else {
        CELL_BORDER = CELL_SIZE;
    }
};

// Method to set grid size based on cell size 
const setCanvasSizeFull = () => {
    let gridSize = Math.floor(0.9 * window.innerHeight / CELL_BORDER);
    width = gridSize;
    height = gridSize;

    let canvasHeight = gridSize * CELL_BORDER + 1;
    canvas.height = canvasHeight;
    canvas.width = canvasHeight;
    universe = Universe.new_rand(width, height);
}

// Render loop, runs each frame
const renderLoop = () => {
    fps.render();
    universe.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};

// Methods for drawing the grid and cells to the canvas
// ----------------------------------------------------
const drawGrid = () => {
    if (!showGrid) {
        return;
    }
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

    // Fill all alive cells
    // (N.B. setting context fillStyle is expensive, so set it once
    // and do all alive cells, then set it again and do all dead,
    // instead of changing for each cell)
    ctx.fillStyle = ALIVE_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            // Only color alive cells at this point
            if (!bitIsSet(idx, cells)) {
                continue;
            }

            ctx.fillRect(
                col * CELL_BORDER + 1,
                row * CELL_BORDER + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    // Fill all dead cells
    ctx.fillStyle = DEAD_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            // Only color dead cells at this point
            if (bitIsSet(idx, cells)) {
                continue;
            }

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

const clearCanvas = () => {
    ctx.clearRect(0, 0, width * CELL_BORDER + 2, height * CELL_BORDER + 2);
}

const clearCanvasRedrawCells = () => {
    clearCanvas();
    if (borderCheckbox.checked) {
        CELL_BORDER = CELL_SIZE + 1;
    }
    else {
        CELL_BORDER = CELL_SIZE;
        setCanvasSizeFull();
    }
    drawCells();
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
});

// Event listener for reset button
resetButton.addEventListener("click", event => {
    universe = Universe.new_rand(width, height);

    // Redraw the scene, in case we're currently paused
    drawGrid();
    drawCells();
});

// Event listener for clear button
clearButton.addEventListener("click", event => {
    universe = Universe.new(width, height);

    // Redraw the scene, in case we're currently paused
    drawGrid();
    drawCells();
});

// Event listener for grid button 
gridButton.addEventListener("click", event => {
    showGrid = !showGrid;
    if (!showGrid) {
        clearCanvasRedrawCells();
    }
    else {
        clearCanvas();
        CELL_BORDER = CELL_SIZE + 1;
        setCanvasSizeFull();
        drawGrid();
        drawCells();
    }
});

// Event listener for cell border checkbox
borderCheckbox.addEventListener("click", event => {
    if (!showGrid) {
        clearCanvasRedrawCells();
    }
});

// Event listener for the cell size select dropdown
cellSizeSelect.addEventListener("change", event => {
    clearCanvas();
    setCellSize();
    setCanvasSizeFull();
    drawGrid();
    drawCells();
});

// Event listener for the pattern select dropdown
patternSelect.addEventListener("change", event => {
    switch (patternSelect.value) {
        case "blinker":
            pattern = Pattern.blinker();
            break;
        case "toad":
            pattern = Pattern.toad();
            break;
        case "beacon":
            pattern = Pattern.beacon();
            break;
        case "pulsar":
            pattern = Pattern.pulsar();
            break;
        case "pentadeca":
            pattern = Pattern.pentadecathlon();
            break;
        case "glider":
            pattern = Pattern.glider();
            break;
        case "lwss":
            pattern = Pattern.lightweight_spaceship();
            break;
        case "mwss":
            pattern = Pattern.midweight_spaceship();
            break;
        case "hwss":
            pattern = Pattern.heavyweight_spaceship();
            break;
        case "r_pent":
            pattern = Pattern.r_pentomino();
            break;
        case "diehard":
            pattern = Pattern.diehard();
            break;
        case "gosp_gun":
            pattern = Pattern.gosper_glider_gun();
            break;
        case "min_engine":
            pattern = Pattern.minimal_block_engine();
            break;
        case "small_engine":
            pattern = Pattern.small_block_engine();
            break;
        case "lin_engine":
            pattern = Pattern.linear_engine();
            break;
        case "eater1":
            pattern = Pattern.eater_one();
            break;
        default:
            pattern = null;
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
        let angle = parseInt(rotation.value);
        universe.insert_pattern(pattern, row, col, angle);
    }

    // Redraw the scene (most likely we will be toggling cells when the game is paused,
    // so they wouldn't be redrawn until the game was running again otherwise)
    drawGrid();
    drawCells();
});

// --------------------------------------

// Setup cell size and start rendering
setCellSize();
setCanvasSizeFull();
play();