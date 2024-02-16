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
- [x] Connects to remote servers via either WebSockets or HTTP
- [x] Invalid SQL queries are never sent to the server, the client uses the same parser the server uses
- [x] Connections have auto-reconnect capabilities
- [x] Range queries
- [x] Consistent API across all supported protocols and storage engines
- [x] Closely matches the API of the Javascript library
- [x] Asynchronous, lock-free connections

<h2><img height="20" src="https://github.com/surrealdb/surrealdb/blob/main/img/gettingstarted.svg?raw=true">&nbsp;&nbsp;Getting started</h2>

It is important to know what the `surrealdb.wasm` library is, and what it is not:
- It's a library targeted to browsers, not NodeJS.
- It is targeted towards ES modules (`import` statements), not CommonJS (`require` function).

The library comes in three formats:
- **Full**: Being the default, all the features are enabled in this build at the cost of a slightly bigger binary.
	- In-memory
	- IndxDB
	- Connect over WebSockets
	- Connect over HTTP
- **WS**: With this build you can only connect over WebSockets.
- **HTTP**: With this build you can only connect over the REST HTTP api.

## Importing the module
A few code snippets to showcase various ways of importing the library.

```js
import { Surreal } from 'surrealdb.wasm';
import { Surreal } from 'surrealdb.wasm/ws';
import { Surreal } from 'surrealdb.wasm/http';
```

### Via UNPKG
```js
import { Surreal } from 'https://unpkg.com/surrealdb.wasm/lib/full.js';
import { Surreal } from 'https://unpkg.com/surrealdb.wasm/lib/ws.js';
import { Surreal } from 'https://unpkg.com/surrealdb.wasm/lib/http.js';
```


<h2><img height="20" src="https://github.com/surrealdb/surrealdb/blob/main/img/features.svg?raw=true">&nbsp;&nbsp;Quick look</h2>

This library enables simple and advanced querying of an embedded or remote database. By default, all remote connections to SurrealDB are made over WebSockets, and automatically reconnect when the connection is terminated.

```js
import { Surreal } from 'surrealdb.wasm';

const db = new Surreal();

try {
	// Connect to the database
	await db.connect("ws://127.0.0.1:8000");

	// Signin as a namespace, database, or root user
	await db.signin({
		username: "root",
		password: "root",
	});

	// Select a specific namespace / database
	await db.use({ namespace: "test", database: "test" });

	// Create a new person with a random id
	let created = await db.create("person", {
		title: "Founder & CEO",
		name: {
			first: "Tobie",
			last: "Morgan Hitchcock",
		},
		marketing: true,
		identifier: Math.random().toString(36).substr(2, 10),
	});

	// Update a person record with a specific id
	let updated = await db.merge("person:jaime", {
		marketing: true
	});

	// Select all people records
	let people = await db.select("person");

	// Perform a custom advanced query
	let groups = await db.query(
		"SELECT marketing, count() FROM type::table($table) GROUP BY marketing",
		{
			table: "person",
		},
	);

	// Delete all people upto but not including Jaime
	let deleted = await db.delete("person:..jaime");

	// Delete all people
	await db.delete("person");
} catch (e) {
	console.error("ERROR", e);
}
```

### Live Query API
The live query API is very similar to that of the `select` method. The only differences are that it uses `live`
in place of `select` and it returns [ReadableStream](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream).

```js
// Live select all records from a table
const stream = await db.live('person');

// Live select a range records from a table
const stream = await db.live('person:jane..john');

// Live select a specific record from a table
const stream = await db.live('person:jane');

// Get a reader
const reader = stream.getReader();

// Listen for changes
while (true) {
	// Read from the stream
	const { done, value } = await reader.read();

	// Exit the loop if done
	if (done) break;

	// Do something with each notification
	console.log(value);
}
```
