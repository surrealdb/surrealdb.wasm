# surrealdb.wasm

The official SurrealDB library for WebAssembly.

[![](https://img.shields.io/badge/status-beta-ff00bb.svg?style=flat-square)](https://github.com/surrealdb/surrealdb.wasm)
[![](https://img.shields.io/badge/docs-view-44cc11.svg?style=flat-square)](https://surrealdb.com/docs/integration/libraries/webassembly)
[![](https://img.shields.io/badge/license-Apache_License_2.0-00bfff.svg?style=flat-square)](https://github.com/surrealdb/surrealdb.wasm)
[![](https://img.shields.io/npm/v/surrealdb.wasm?style=flat-square)](https://www.npmjs.com/package/surrealdb.wasm)


<h2><img height="20" src="https://github.com/surrealdb/surrealdb/raw/main/img/whatissurreal.svg?raw=true">&nbsp;&nbsp;What is SurrealDB?</h2>

SurrealDB is an end-to-end cloud-native database designed for modern applications, including web, mobile, serverless, Jamstack, backend, and traditional applications. With SurrealDB, you can simplify your database and API infrastructure, reduce development time, and build secure, performant apps quickly and cost-effectively.

**Key features of SurrealDB include:**

- **Reduces development time**: SurrealDB simplifies your database and API stack by removing the need for most server-side components, allowing you to build secure, performant apps faster and cheaper.
- **Real-time collaborative API backend service:** SurrealDB functions as both a database and an API backend service, enabling real-time collaboration.
- **Support for multiple querying languages:** SurrealDB supports SQL querying from client devices, GraphQL, ACID transactions, WebSocket connections, structured and unstructured data, graph querying, full-text indexing, and geospatial querying.
- **Granular access control**: SurrealDB provides row-level permissions-based access control, giving you the ability to manage data access with precision.


View the [features](https://surrealdb.com/features), the latest [releases](https://surrealdb.com/releases), and [documentation](https://surrealdb.com/docs).

<h2><img height="20" src="https://github.com/surrealdb/surrealdb/blob/main/img/features.svg?raw=true">&nbsp;&nbsp;Features</h2>

- [x] Can be used as an embedded database
- [x] Consistent API across all supported protocols and storage engines
- [x] Interfaces with the JavaScript SDK to have a common interface between all connections
- [x] Asynchronous, lock-free connections

<h2><img height="20" src="https://github.com/surrealdb/surrealdb/blob/main/img/gettingstarted.svg?raw=true">&nbsp;&nbsp;Getting started</h2>

It is important to know what the `surrealdb.wasm` library is, and what it is not:
- It's a library targeted to browsers, not NodeJS.
- It is targeted towards ES modules (`import` statements), not CommonJS (`require` function).

## Importing the module
Here is an example on how to import this package, and how to connect it to the JavaScript SDK

```js
import { Surreal } from 'surrealdb.js';
import { surrealdbWasmEngines } from 'surrealdb.wasm';

const db = new Surreal({
	engines: surrealdbWasmEngines(),
});

// Can now use the JS SDK as you normally would, 
// but with added mem and indxdb protocols
```

<h2><img height="20" src="https://github.com/surrealdb/surrealdb/blob/main/img/features.svg?raw=true">&nbsp;&nbsp;Quick look</h2>

This library enables simple and advanced querying of an embedded or remote database. By default, all remote connections to SurrealDB are made over WebSockets, and automatically reconnect when the connection is terminated.

```js
import { Surreal } from 'surrealdb.js';
import { surrealdbWasmEngines } from 'surrealdb.wasm';

const db = new Surreal({
	engines: surrealdbWasmEngines(),
});

// Connect to an in-memory database
await db.connect("mem://");
// Connect to an indexeddb database
await db.connect("indxdb://demo");

// Select a specific namespace / database
await db.use({ 
	namespace: "test", 
	database: "test" 
});

// Create a new person with a random id
let created = await db.create("person", {
	title: "Founder & CEO",
	name: {
		first: "Tobie",
		last: "Morgan Hitchcock",
	},
	marketing: true,
});

// Update a person record with a specific id
let updated = await db.merge(new RecordId('person', 'jaime'), {
	marketing: true,
});

// Select all people records
let people = await db.select("person");

// Perform a custom advanced query
let groups = await db.query(
	"SELECT marketing, count() FROM $tb GROUP BY marketing",
	{
		tb: new Table("person"),
	},
);
```
