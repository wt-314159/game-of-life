import { buildShaderProgram, compileShader, getProgramLocations } from "./webgl-utilities.js"

const name = 'webgl';

let gl = null;
let glCanvas = null;

const DEAD_COLOR = [1, 1, 1, 1.0];
const ALIVE_COLOR = [0, 0, 0, 1.0];

let aspectRatio;
let scale = [1.0, 1.0];

// Vertex info
let vertexBuffer;
let vertexNumComponents = 2;
let vertexCount;
let vao;

let shaderProgram;
let programInfo;

function startup() {
    glCanvas = document.getElementById("game-layer");
    gl = glCanvas.getContext("webgl2");

    const shaderSet = [
        { 
            type: gl.VERTEX_SHADER,
            source: document.getElementById("vertex-shader").firstChild.nodeValue,
        },
        {
            type: gl.FRAGMENT_SHADER,
            source: document.getElementById("fragment-shader").firstChild.nodeValue,
        }
    ];

    shaderProgram = buildShaderProgram(gl, shaderSet);
    programInfo = getProgramLocations(gl, shaderProgram);
    
    aspectRatio = glCanvas.width / glCanvas.height;
    scale = [0.5, 0.5];

    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    vertexBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);
    setPositionAttribute();
}

function onFrame() {
    gl.viewport(0, 0, glCanvas.width, glCanvas.height);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.useProgram(shaderProgram);
    let vertexCount = setSquare(60);
    gl.bindVertexArray(vao);
    gl.uniform2f(programInfo.uniformLocations.uResolution, gl.canvas.width, gl.canvas.height);
    gl.uniform2f(programInfo.uniformLocations.uTranslation, 30, 80);
    gl.uniform4fv(programInfo.uniformLocations.uGlobalColor, [0.1, 0.7, 0.2, 1.0]);

    gl.drawArrays(gl.TRIANGLE_STRIP, 0, vertexCount)
}

function drawCellsFrame(width, height, cells, cellSize, cellBorderSize) {
    gl.viewport(0, 0, glCanvas.width, glCanvas.height);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.useProgram(shaderProgram);
    gl.bindVertexArray(vao);

    gl.uniform2f(programInfo.uniformLocations.uResolution, gl.canvas.width, gl.canvas.height);

    // Do alive cells first
    gl.uniform4fv(programInfo.uniformLocations.uGlobalColor, ALIVE_COLOR);
    let idx = 0;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            idx += 1;
            if (!bitIsSet(idx, cells)) {
                continue;
            }
            gl.uniform2f(programInfo.uniformLocations.uTranslation, col * cellBorderSize + 1, row * cellBorderSize + 1);
            gl.drawArrays(gl.TRIANGLE_STRIP, 0, vertexCount);
        }
    }

    // Do dead cells next
    gl.uniform4fv(programInfo.uniformLocations.uGlobalColor, DEAD_COLOR);
    idx = 0;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            idx += 1;
            if (bitIsSet(idx, cells)) {
                continue;
            }
            gl.uniform2f(programInfo.uniformLocations.uTranslation, col * cellBorderSize + 1, row * cellBorderSize + 1);
            gl.drawArrays(gl.TRIANGLE_STRIP, 0, vertexCount);
        }
    }
}

function clearCellsCanvas() {
    gl.viewport(0, 0, glCanvas.width, glCanvas.height);
    gl.clear(gl.COLOR_BUFFER_BIT);
}

function setSquare(size) {
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([
        0, 0,
        size, 0,
        0, size,
        size, size
    ]), gl.STATIC_DRAW);

    return 4;
}

function setSquareSize(squareSize) {
    vertexCount = setSquare(squareSize);
}

function setRectangle(x, y, width, height) {
    var x2 = x + width;
    var y2 = y + height;

    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([
        x, y,
        x2, y,
        x, y2,
        x2, y2
    ]), gl.STATIC_DRAW);

    return 4;
}

function setPositionAttribute() {
    gl.createVertexArray();
    gl.bindVertexArray(vao);
    gl.enableVertexAttribArray(programInfo.attribLocations.vertexPosition);
    var size = vertexNumComponents;     // 2 components per vertex
    var type = gl.FLOAT;                // the data is 32 bit floats
    var normalize = false;              // don't normalize the data
    var stride = 0;                     // 0 = stride between vertices is calculated from size * sizeof(type)
    var offset = 0;                     // start at beginning of buffer
    gl.vertexAttribPointer(
        programInfo.attribLocations.vertexPosition,
        size,
        type,
        normalize,
        stride,
        offset
    );
    // N.B. gl.vertexAttribPointer also binds current ARRAY_BUFFER to the attribute,
    // so we can now bind something else to ARRAY_BUFFER if we want, the attribute
    // will continue to use vertexBuffer
}

const getIndex = (row, column, width) => {
    return row * width + column;
};

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};

export { startup, onFrame, drawCellsFrame, clearCellsCanvas, setSquareSize };