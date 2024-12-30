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

// instance information
let maxCells;
let instancePositions;
let instancePositionBuffer;
let instanceColors;
let instanceColorBuffer;

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
    gl.useProgram(shaderProgram);
    
    aspectRatio = glCanvas.width / glCanvas.height;
    scale = [0.5, 0.5];

    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    vertexBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);
    setPositionAttribute();

    instancePositionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, instancePositionBuffer);
    setInstancePositionAttribute();

    instanceColorBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, instanceColorBuffer);
    setInstanceColorAttribute();

    gl.uniform2f(programInfo.uniformLocations.resolution, gl.canvas.width, gl.canvas.height);
    gl.uniform4fv(programInfo.uniformLocations.aliveColor, ALIVE_COLOR);
    gl.uniform4fv(programInfo.uniformLocations.deadColor, DEAD_COLOR);
}

function drawCellsFrame(width, height, cells, cellSize, cellBorderSize) {
    gl.viewport(0, 0, glCanvas.width, glCanvas.height);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.useProgram(shaderProgram);
    gl.bindVertexArray(vao);

    let idx = 0;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            idx += 1;
            
            instanceColors[idx] = bitIsSet(idx, cells) ? 1 : 0;
            let doubleIdx = idx * 2;
            instancePositions[doubleIdx] = col * cellBorderSize + 1;
            instancePositions[doubleIdx + 1] = row * cellBorderSize + 1;
        }
    }

    gl.bindBuffer(gl.ARRAY_BUFFER, instancePositionBuffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, instancePositions);

    gl.bindBuffer(gl.ARRAY_BUFFER, instanceColorBuffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, instanceColors);
    
    const instanceCount = instancePositions.length / 2;
    gl.drawArraysInstanced(gl.TRIANGLE_STRIP, 0, 4, instanceCount);
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

function onGridSizeChanged(width, height) {
    maxCells = width * height;
    instancePositions = new Float32Array(maxCells * 2);
    gl.bindBuffer(gl.ARRAY_BUFFER, instancePositionBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, instancePositions.byteLength, gl.DYNAMIC_DRAW);

    instanceColors = new Float32Array(maxCells);
    gl.bindBuffer(gl.ARRAY_BUFFER, instanceColorBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, instanceColors.byteLength, gl.DYNAMIC_DRAW);

    gl.uniform2f(programInfo.uniformLocations.resolution, gl.canvas.width, gl.canvas.height);
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

function setInstancePositionAttribute() {
    gl.bindBuffer(gl.ARRAY_BUFFER, instancePositionBuffer);
    gl.enableVertexAttribArray(programInfo.attribLocations.instancePosition);
    gl.vertexAttribPointer(
        programInfo.attribLocations.instancePosition,
        2,
        gl.FLOAT,
        false,
        0,
        0
    );
    gl.vertexAttribDivisor(programInfo.attribLocations.instancePosition, 1);    // one value per instance
}

function setInstanceColorAttribute() {
    gl.bindBuffer(gl.ARRAY_BUFFER, instanceColorBuffer);
    gl.enableVertexAttribArray(programInfo.attribLocations.instanceColor);
    gl.vertexAttribPointer(
        programInfo.attribLocations.instanceColor,
        1,
        gl.FLOAT,
        false,
        0,
        0
    );
    gl.vertexAttribDivisor(programInfo.attribLocations.instanceColor, 1);
}

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};

export { startup, drawCellsFrame, clearCellsCanvas, setSquareSize, onGridSizeChanged };