import Surreal from "surrealdb.js";
import { surrealdbWasmEngines } from "./lib/embedded";

const surreal = new Surreal({
	engines: surrealdbWasmEngines(),
});

console.log(await surreal.connect('mem://', {
	namespace: 'test',
	database: 'test'
}));
console.log(await surreal.create('test'));

await surreal.close();
