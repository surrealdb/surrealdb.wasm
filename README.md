<br>

<p align="center">
    <img width=120 src="https://raw.githubusercontent.com/surrealdb/icons/main/surreal.svg" />
    &nbsp;
    <img width=120 src="https://raw.githubusercontent.com/surrealdb/icons/main/webassembly.svg" />
</p>

<h3 align="center">A WebAssembly engine for the SurrealDB JavaScript SDK.</h3>

<br>

<p align="center">
    <a href="https://github.com/surrealdb/surrealdb.wasm"><img src="https://img.shields.io/badge/status-stable-ff00bb.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://surrealdb.com/docs/sdk/javascript"><img src="https://img.shields.io/badge/docs-view-44cc11.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://www.npmjs.com/package/@surrealdb/wasm"><img src="https://img.shields.io/npm/v/@surrealdb/wasm?style=flat-square"></a>
    &nbsp;
    <a href="https://www.npmjs.com/package/@surrealdb/wasm"><img src="https://img.shields.io/npm/dm/@surrealdb/wasm?style=flat-square"></a>
</p>

<p align="center">
    <a href="https://surrealdb.com/discord"><img src="https://img.shields.io/discord/902568124350599239?label=discord&style=flat-square&color=5a66f6"></a>
    &nbsp;
    <a href="https://twitter.com/surrealdb"><img src="https://img.shields.io/badge/twitter-follow_us-1d9bf0.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://www.linkedin.com/company/surrealdb/"><img src="https://img.shields.io/badge/linkedin-connect_with_us-0a66c2.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://www.youtube.com/@SurrealDB"><img src="https://img.shields.io/badge/youtube-subscribe-fc1c1c.svg?style=flat-square"></a>
</p>

# @surrealdb/wasm

A WebAssembly engine for the SurrealDB [JavaScript SDK](https://github.com/surrealdb/surrealdb.js).

This library is a plugin for the SurrealDB JavaScript SDK, which can be used to run SurrealDB as an embedded database within a browser environment, not server side environments.

It enables SurrealDB to be run in-memory, or to persist data by running on top of IndexedDB. It allows for a consistent JavaScript and TypeScript API when using the `surrealdb.js` library by adding support for embedded storage engines (`memory`, `indxdb`) alongside the remote connection protocols (`http`, `https`, `ws`, `wss`).

This library works with ES modules (`import`), not CommonJS (`require`).

## Example usage

```js
import { Surreal } from "surrealdb";
import { surrealdbWasmEngines } from "@surrealdb/wasm";

// Enable the WebAssembly engines
const db = new Surreal({
    engines: surrealdbWasmEngines(),
});

// Now we can start SurrealDB as an in-memory database
await db.connect("mem://");
// Or we can start a persisted IndexedDB database
await db.connect("indxdb://demo");

// Now use the JavaScript SDK as normal.
```

## Usage with Vite

When using [Vite](https://vitejs.dev/) the following configuration is recommended to be placed in your `vite.config.ts`

```js
optimizeDeps: {
    exclude: ["@surrealdb/wasm"],
    esbuildOptions: {
        target: "esnext",
    },
},
esbuild: {
    supported: {
        "top-level-await": true
    },
}
```
