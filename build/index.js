import * as esbuild from "esbuild";
import copyFilePlugin from "esbuild-plugin-copy-file";
import { spawn } from "node:child_process";
import fs from "node:fs";

const shimContent = new Buffer.from(fs.readFileSync("./build/shim.js"));
const targets = ["surreal"];

await Promise.all(targets.map(build));

async function build(target) {
    await applyPatches(target);
    await bundle(target);
}

async function applyPatches(target) {
    let content = fs.readFileSync(`compiled/${target}/index.js`).toString();
    content = shimContent + content;

    const tauriPatch = fs
        .readFileSync("build/tauri.patch")
        .toString()
        .split("===========\n");
	
    content = content.replace(tauriPatch[0], tauriPatch[1]);

    fs.writeFileSync(`compiled/${target}/index.js`, content);
}

async function bundle(target) {
    await esbuild.build({
        entryPoints: [`lib/${target}.ts`],
        bundle: true,
        minifyWhitespace: true,
        minifySyntax: true,
        format: "esm",
        platform: "node",
        outfile: `dist/${target}/index.js`,
        external: ["surrealdb"],
        plugins: [
            copyFilePlugin({
                after: Object.fromEntries(
                    ["index_bg.wasm", "index_bg.wasm.d.ts", "index.d.ts"].map(
                        (f) => [
                            `dist/${target}/${f}`,
                            `compiled/${target}/${f}`,
                        ],
                    ),
                ),
            }),
        ],
    });

    await esbuild.build({
        entryPoints: [`lib/${target}.ts`],
        bundle: true,
        minifyWhitespace: true,
        minifySyntax: true,
        format: "esm",
        platform: "node",
        outfile: `dist/${target}/index.bundled.js`,
        plugins: [
            copyFilePlugin({
                after: Object.fromEntries(
                    ["index_bg.wasm", "index_bg.wasm.d.ts", "index.d.ts"].map(
                        (f) => [
                            `dist/${target}/${f}`,
                            `compiled/${target}/${f}`,
                        ],
                    ),
                ),
            }),
        ],
    });

    spawn("npx", [
        "dts-bundle-generator",
        "-o",
        `dist/${target}/types.d.ts`,
        `lib/${target}.ts`,
        "--no-check",
        "--export-referenced-types",
        "false",
    ]);
}
