import * as wasm from "ghostly_game_web";

const renderLoop = () => {
	wasm.main_loop();
	requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);