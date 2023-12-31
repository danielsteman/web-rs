% id: 4
% title: WASM with Rust and Javascript 🦀
% date: 2022-12-06
% tags: web

WebAssembly (WASM) is a new approach towards web development, which leverages the speed and robustness of lower level languages such as C, C++ and Rust to power websites. Today I started following [this book](https://rustwasm.github.io/docs/book/game-of-life/setup.html) and felt like writing about my experience with WASM. `wasm-bindgen` is a crate (which is was packages in Rust are called) that is used to interface with Javascript and `wasm-pack` compiles Rust code to WASM. To expose functions in Rust you can add the `#[wasm-bind]` annotation on top of the function signature.

Build your Rust project, which incorporates `wasm-bindgen` as a dependency with `wasm-pack`:

```bash
wasm-pack build
```

The command above generates a `pkg/` with artifacts. `create-wasm-app` is a Javascript scaffolding tool that makes it easier to setup a website that incorporates WASM. `www` is passed as an argument and will be the name of the folder containing the boilerplate code:

```bash
npm init wasm-app www
```

The built package can be used in Javascript and for local development, we can simply add a reference to the `package.json` that is generated by the command above. The reference would look something like this:

```json
"dependencies": {
  "wasm-project": "file:../pkg"
}
```

Now it's possible to import Rust functions in javascript. To test, you can create a simple function and import it in the `index.js` that was generated for you:

```rust
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, world!");
}
```

```js
import * as wasm from "wasm-project";
```

The [book](https://rustwasm.github.io/docs/book/game-of-life/setup.html) implements an animation of [the game of life](https://playgameoflife.com/), for which Google has an [easter egg](https://www.google.com/search?q=game+of+life+) as I discovered while writing this, which consists of a lot of calculations that can be performed more efficiently with a low-level language such as Rust.
