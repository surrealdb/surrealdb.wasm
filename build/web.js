import * as esbuild from "esbuild";

const targets = ['embedded'];
await Promise.all(targets.map(bundle));

async function bundle(target) {
	await esbuild.build({
		entryPoints: [`lib/${target}.js`],
		sourcemap: true,
		bundle: true,
		format: "esm",
		platform: 'node',
		outfile: `dist/${target}/web.js`,
	});
}
