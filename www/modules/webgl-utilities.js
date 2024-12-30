const name = 'webgl-utilities';

// Builds and links a shader program
function buildShaderProgram(gl, shaderInfo) {
    const program = gl.createProgram();

    shaderInfo.forEach((info) => {
        const shader = compileShader(gl, info.type, info.source);

        if (shader) {
            gl.attachShader(program, shader);
        }
    });

    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        console.log("Error linking shader program");
        console.log(gl.getProgramInfoLog(program));
    }

    return program;
}

function compileShader(gl, type, source) {
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

function getProgramLocations(gl, program) {
    var programInfo = {
        program: program,
        attribLocations: {
            vertexPosition: gl.getAttribLocation(program, "aVertexPosition"),
        },
        uniformLocations: {
            uResolution: gl.getUniformLocation(program, "uResolution"),
            uTranslation: gl.getUniformLocation(program, "uTranslation"),
            uGlobalColor: gl.getUniformLocation(program, "uGlobalColor"),
        }
    };
    return programInfo;
}

export { buildShaderProgram, compileShader, getProgramLocations }