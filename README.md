<br>

<p align="center">
    <img width=120 src="https://raw.githubusercontent.com/surrealdb/icons/main/surreal.svg" />
    &nbsp;
    <img width=120 src="https://raw.githubusercontent.com/surrealdb/icons/main/webassembly.svg" />
</p>

<h3 align="center">A WebAssembly engine for the SurrealDB JavaScript SDK.</h3>

<br>

<p align="center">
    <a href="https://github.com/surrealdb/surrealdb.wasm"><img src="https://img.shields.io/badge/status-beta-ff00bb.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://surrealdb.com/docs/integration/libraries/javascript"><img src="https://img.shields.io/badge/docs-view-44cc11.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://github.com/surrealdb/surrealdb.wasm"><img src="https://img.shields.io/npm/v/surrealdb.wasm?style=flat-square"></a>
</p>

<p align="center">
    <a href="https://surrealdb.com/discord"><img src="https://img.shields.io/discord/902568124350599239?label=discord&style=flat-square&color=5a66f6"></a>
    &nbsp;
    <a href="https://twitter.com/surrealdb"><img src="https://img.shields.io/badge/twitter-follow_us-1d9bf0.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://www.linkedin.com/company/surrealdb/"><img src="https://img.shields.io/badge/linkedin-connect_with_us-0a66c2.svg?style=flat-square"></a>
    &nbsp;
    <a href="https://www.youtube.com/channel/UCjf2teVEuYVvvVC-gFZNq6w"><img src="https://img.shields.io/badge/youtube-subscribe-fc1c1c.svg?style=flat-square"></a>
</p>

# surrealdb.wasm

A WebAssembly engine for the SurrealDB JavaScript SDK.

- [x] A plugin for the SurrealDB JavaScript SDK, which can be used to run SurrealDB as an embedded database within a browser environment.
- [x] Enables SurrealDB to be run in-memory, or to persist data by running on top of IndexedDB.
- [x] It enables a consistent JavaScript and TypeScript API from the `surrealdb.js` library, with support for any protocol (`http:`, `https:`, `ws:`, `wss:`) or storage engine (`memory`, `indxdb:`).
- [x] It's a library targeted to browsers, not server side environments.
- [x] It works with ES modules (`import` statements), not CommonJS (`require` function).

## Example usage

```js
import { Surreal } from 'surrealdb.js';
import { surrealdbWasmEngines } from 'surrealdb.wasm';

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
