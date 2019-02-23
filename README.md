A 3D cellular automaton simulator using WebAssembly and WebGL.

![Screenshot](screenshots/screenshot.png?raw=true)

Right now this simulates a life-like CA (B6,8/S5,6,7,13), but I haven't yet found any CAs that demonstrate behavior as interesting as that of 2D CAs like Life (B3/S23).

To run this, [run the online version](https://nstoddard.github.io/ca-3d/), or compile it yourself:
  * Install [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen), [`wasm-opt`](https://github.com/WebAssembly/binaryen), and [`terser`](https://github.com/terser-js/terser).
  * Run `./build-release.sh`.
  * Use a web server to serve the `html/` directory.

To experiment with different CA rules, modify the `update()` method in `src/ca.rs`. A couple of interesting rules are: B6,7/S5,7,8 (has a diagonal spaceship, but quickly expands to fill the whole space), B6,9/S5,6,7,10 (has a similar spaceship, but it seems somewhat more rare), B5/S4,5,10 (which takes a while to settle into still lifes and oscillators).

Not yet implemented:
  * Better rendering performance
  * Controls to move the camera around
  * Multiple cellular automata rules to choose from, without having to recompile
