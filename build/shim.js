const Deno =
	globalThis.Deno ?? (
		typeof global === 'undefined'
			? undefined
			: {
				readFile: (await import('node:fs')).readFileSync
			}
	)

const module = await (async () => {
	const crypto = globalThis.crypto ?? await import('node:crypto');
	return {
		require: (string) => {
			if(string !== "crypto") throw new Error("Unexpected require " + string)
			return crypto
		}
	};
})();
