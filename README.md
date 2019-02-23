A 3D cellular automaton simulator using WebAssembly and WebGL.

![Screenshot](screenshots/screenshot.png?raw=true)

Right now this simulates a life-like CA (B6,8/S5,6,7,13), but I haven't yet found any CAs that demonstrate behavior as interesting as that of life (B3/S23) in 2D.

To compile and run this:
  * Install [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen), [`wasm-opt`](https://github.com/WebAssembly/binaryen), and [`terser`](https://github.com/terser-js/terser).
  * Run `./build-release.sh`.
  * Use a web server to serve the `html/` directory.

To experiment with different CA rules, modify the `update()` method in `src/ca.rs`.

Not yet implemented:
  * Better rendering performance
  * Controls to move the camera around
  * Multiple cellular automata rules to choose from
