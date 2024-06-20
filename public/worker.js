import init from "/./assets/dioxus/Mars.js";

init("/./assets/dioxus/Mars_bg.wasm").then(wasm => {
  wasm.start_worker();
});
