import * as wasm from "ghostly_game_web";

const renderLoop = () => {
	wasm.main_loop();
	requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

const canvas = document.getElementById("gengar_canvas");
var gl = canvas.getContext("webgl2");

var width = gl.canvas.clientWidth;
var height = gl.canvas.clientHeight;

gl.canvas.width = width;
gl.canvas.height = height;