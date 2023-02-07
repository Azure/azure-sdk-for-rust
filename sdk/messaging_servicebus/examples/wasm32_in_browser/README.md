# `wasm32` in the browser

This example demonstrates how to build for `wasm32-unknown-unknown` and run in the browser.
This will need to use the `wasm-pack` tool to build the project.

## Steps

1. Build the project with `wasm-pack`:

    ```sh
    wasm-pack build --target web
    ```

2. Host the `pkg` directory with a web server. For example, with `python`:

    ```sh
    python -m http.server
    ```

3. Open the URL in a browser. For example, `http://localhost:8000`, and open the developer console to see the output.
