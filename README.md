# RBXplorer
Super early WIP. Explore Roblox model files in the browser. Powered by compiling [rbx-dom](https://github.com/LPGhatguy/rbx-dom) to WebAssembly and binding to it in JS.

## Running
Requirements:
- Rust 1.34+ with `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`)
- Node.js 10.x

To run the project locally:

```sh
npm install
npm run-script serve
```

## License
RBXplorer is available under the terms of the MIT license. See [LICENSE.txt](LICENSE.txt) for details.