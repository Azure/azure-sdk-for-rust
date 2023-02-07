import init from "./pkg/wasm32_in_browser.js";

const run = async () => {
    const wasm = await init("./pkg/wasm32_in_browser_bg.wasm");

    const addResult = wasm.run();
};
run();
