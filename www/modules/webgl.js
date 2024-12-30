import { buildShaderProgram, compileShader, getProgramLocations } from "./webgl-utilities.js"

const name = 'webgl';

let gl = null;
let glCanvas = null;

let aspectRatio;
let scale = [1.0, 1.0];

// Vertex info
let vertices;
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

    createVertexBuffer();
    setPositionAttribute();
}

function onFrame() {
    gl.viewport(0, 0, glCanvas.width, glCanvas.height);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.useProgram(shaderProgram);
    gl.bindVertexArray(vao);
    gl.uniform2f(programInfo.uniformLocations.uResolution, gl.canvas.width, gl.canvas.height);
    gl.uniform4fv(programInfo.uniformLocations.uGlobalColor, [0.1, 0.7, 0.2, 1.0]);

    gl.drawArrays(gl.TRIANGLE_STRIP, 0, vertexCount)
}

function createVertexBuffer() {
    // Create and bind vertex buffer
    vertexBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);

    // create array of vertices for a square
    vertices = [80, 80, 20, 80, 80, 20, 20, 20];
    // Fill vertex buffer with data
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);

    vertexCount = vertices.length / vertexNumComponents;
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

export { startup, onFrame };