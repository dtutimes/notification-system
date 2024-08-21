# notification-system

Web scraper of the DTU Website as a base to create the notification alerts system

## Build

For building binary which prints `json` response, use the following command:

```bash
cargo build --release
```

For wasm build (to use it as `npm` package), build with the following command

```bash
wasm-pack build -t nodejs --features wasm
```

## Installation

Install as a binary

```bash
cargo install --path .
```

Install as npm package

```bash
wasm-pack build -t nodejs --features wasm # this builds and output a pkg/ directory, copy this to root of your node project

# with pkg/ folder in root of your node project
npm i pkg
```
