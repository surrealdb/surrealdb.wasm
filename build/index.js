import * as esbuild from "esbuild";
import copyFilePlugin from 'esbuild-plugin-copy-file';
import fs from 'fs';

const shimContent = new Buffer.from(await fs.readFileSync('./build/shim.js'));
const targets = ['full', 'ws', 'http'];

await Promise.all(targets.map(build));

async function build(target) {
	await applyShim(target);
	await bundle(target);
}

async function applyShim(target) {
	let content = fs.readFileSync(`compiled/${target}/index.js`);
	content = shimContent + content;
	fs.writeFileSync(`compiled/${target}/shimmed.js`, content)
}

async function bundle(target) {
	await esbuild.build({
		entryPoints: [`compiled/${target}/shimmed.js`],
		sourcemap: true,
		bundle: true,
		format: "esm",
		platform: 'node',
		outfile: `dist/${target}/index.js`,
		plugins: [
			copyFilePlugin({
				after: Object.fromEntries(
					["index_bg.wasm", "index_bg.wasm.d.ts", "index.d.ts"].map(
						(f) => [
							`dist/${target}/${f}`,
							`compiled/${target}/${f}`,
						]
					)
				),
			}),
		],
	});
}
