// We need a file reading API that works with NodeJS. This API is not used in the browser, but is used in Deno and Node.
// Use the global Deno variable if already available
// Otherwise create a custom object for the readFile function which uses Node's "fs" api under the hood.
const Deno =
	globalThis.Deno ?? (
		typeof global === 'undefined'
			? undefined
			: {
				readFile: (await import('node:fs')).readFileSync
			}
	)

// Provide a custom require function to the WASM engine. If the crypto module is not available globally, we need to pass it though manually
const module = await (async () => {
	const crypto = globalThis.crypto ?? await import('node:crypto');
	return {
		require: (string) => {
			if(string !== "crypto") throw new Error("Unexpected require " + string)
			return crypto
		}
	};
})();
