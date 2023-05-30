// @deno-types="../dist/full/index.d.ts"
import { Surreal } from "../dist/full/index.js";

Object.defineProperty(Surreal, "ENDPOINTS", {
	value: {
		"AS": "wss://cloud.as.surrealdb.com",
		"EU": "wss://cloud.eu.surrealdb.com",
		"US": "wss://cloud.us.surrealdb.com",
	},
	writable: false,
	enumerable: false,
	configurable: false,
});

export { Surreal, Surreal as default };
