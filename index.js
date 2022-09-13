export default import(/* webpackChunkName: "surrealdb" */ './pkg/surrealdb.js').catch(console.error).then(async module => {

	Object.defineProperty(module.Surreal, 'ENDPOINTS', {
		value: {
			"AS": "wss://cloud.as.surrealdb.com/rpc",
			"EU": "wss://cloud.eu.surrealdb.com/rpc",
			"US": "wss://cloud.us.surrealdb.com/rpc",
		},
		writable: false,
		enumerable: false,
		configurable: false,
	});

	Object.defineProperty(module.Surreal, 'Instance', {
		value: new module.Surreal(),
		writable: false,
		enumerable: false,
		configurable: false,
	});

	if (typeof window !== 'undefined') {
		window.Surreal = module.Surreal;
	}

	if (typeof global !== 'undefined') {
		global.Surreal = module.Surreal;
	}

	module.setup();

	return module.Surreal;

});
