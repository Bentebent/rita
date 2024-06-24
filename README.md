# RITA

# Requirements

- Cargo (stable and nightly)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Testing WASM

- Download nginx
- Copy nginx.conf to the conf directory in your nginx install
- Replace the commented # root with the path to the root folder of this repository
  - Example: root C:/projects/rita;
- Open cmd in the nginx root dir
- Run command: start nginx
- Browse to localhost:9001
