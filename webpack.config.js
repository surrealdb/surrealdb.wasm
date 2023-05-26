export default [
	{
		target: 'web',
		mode: 'production',
		entry: '/index.js',
		output: {
			clean: true,
			filename: 'index.js',
			globalObject: 'window',
			path: new URL('./dist/web', import.meta.url).pathname,
			library: {
				name: 'Surreal',
				type: 'window',
				export: 'default',
				umdNamedDefine: true,
			},
		},
		devServer: {
			compress: true,
			client: {
				overlay: {
					errors: true,
					warnings: true,
				},
			},
			static: {
				directory: process.cwd(),
			},
			watchFiles: [
				'index.html',
				'index.js',
				'pkg/*',
			],
		},
		experiments: {
			asyncWebAssembly: true,
		},
		devtool: false,
	},
	{
		target: 'node',
		mode: 'production',
		entry: './index.js',
		output: {
			clean: true,
			filename: 'index.cjs',
			path: new URL('./dist/lib', import.meta.url).pathname,
			library: {
				name: 'Surreal',
				type: 'umd',
				export: 'default',
				umdNamedDefine: true,
			},
		},
		experiments: {
			asyncWebAssembly: true,
		},
		devtool: false,
	},
];
