import * as esbuild from "esbuild";
import copyFilePlugin from 'esbuild-plugin-copy-file';
import fs from 'fs';

const shimContent = new Buffer.from(await fs.readFileSync('./build/shim.js'));
const targets = ['embedded'];

await Promise.all(targets.map(build));

async function build(target) {
	await applyPatches(target);
	await bundle(target);
}

async function applyPatches(target) {
	let content = fs.readFileSync(`compiled/${target}/index.js`).toString();
	content = shimContent + content;

	const tauriPatch = fs.readFileSync(`build/tauri.patch`).toString().split("===========\n");
	content = content.replace(tauriPatch[0], tauriPatch[1]);

	fs.writeFileSync(`compiled/${target}/patched.js`, content)
}

async function bundle(target) {
	await esbuild.build({
		entryPoints: [`compiled/${target}/patched.js`],
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
