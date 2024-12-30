const name = 'webgl';

let gl = null;
let glCanvas = null;

let aspectRatio;
let scale = [1.0, 1.0];

// Vertex info
let vertexArray;
let vertexBuffer;
let vertexNumComponents = 2;
let vertexCount;

let shaderProgram;
let programInfo;

function startup() {
    glCanvas = document.getElementById("game-layer");
    gl = glCanvas.getContext("webgl2");

    const shaderSet = [
        { 
            type: gl.VERTEX_SHADER,
            id: "vertex-shader",
        },
        {
            type: gl.FRAGMENT_SHADER,
            id: "fragment-shader",
        }
    ];

    shaderProgram = buildShaderProgram(shaderSet);
    
    aspectRatio = glCanvas.width / glCanvas.height;
    scale = [0.5, 0.5];

    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    vertexBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);

    // create array of vertices for a square
    vertexArray = [0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, -0.5];
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertexArray), gl.STATIC_DRAW);

    vertexCount = vertexArray.length / vertexNumComponents;

    setPositionAttribute();
}

function buildShaderProgram(shaderInfo) {
    const program = gl.createProgram();

    shaderInfo.forEach((info) => {
        const shader = compileShader(info.id, info.type);

        if (shader) {
            gl.attachShader(program, shader);
        }
    });

    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        console.log("Error linking shader program");
        console.log(gl.getProgramInfoLog(program));
    }

    programInfo = {
        program: program,
        attribLocations: {
            vertexPosition: gl.getAttribLocation(program, "aVertexPosition"),
        },
        uniformLocations: {
            uScalingFactor: gl.getUniformLocation(program, "uScalingFactor"),
            uGlobalColor: gl.getUniformLocation(program, "uGlobalColor"),
        }
    };

    return program;
}

function compileShader(id, type) {
    const source = document.getElementById(id).firstChild.nodeValue;
    const shader = gl.createShader(type);

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        console.log(
            `Error compiling ${
                type === gl.VERTEX_SHADER ? "vertex" : "fragment"
            } shader:`
        );
        console.log(gl.getShaderInfoLog(shader));
    }
    return shader;
}

function onFrame() {
    gl.viewport(0, 0, glCanvas.width, glCanvas.height);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.useProgram(shaderProgram);
    gl.uniform2fv(programInfo.uniformLocations.uScalingFactor, scale);
    gl.uniform4fv(programInfo.uniformLocations.uGlobalColor, [0.1, 0.7, 0.2, 1.0]);

    gl.drawArrays(gl.TRIANGLE_STRIP, 0, vertexCount)
}

function setPositionAttribute() {
    gl.enableVertexAttribArray(programInfo.attribLocations.vertexPosition);
    gl.vertexAttribPointer(
        programInfo.attribLocations.vertexPosition,
        vertexNumComponents,
        gl.FLOAT,
        false,
        0,
        0
    );
}

export { startup, onFrame };