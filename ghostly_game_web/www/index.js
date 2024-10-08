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

window.addEventListener('keydown', function (event) {
	wasm.key_down(event);
});

window.addEventListener('keyup', function (event) {
	wasm.key_up(event);
});

window.addEventListener('mousemove', function (event) {
	wasm.mouse_move(event);
});

window.addEventListener('mousedown', function (event) {
	wasm.mouse_down(event);
});

window.addEventListener('mouseup', function (event) {
	wasm.mouse_up(event);
});