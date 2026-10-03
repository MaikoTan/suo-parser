// Guards against publishing a tarball that is missing the compiled wasm.
//
// npm falls back to .gitignore rules when there is no .npmignore, and
// wasm-pack writes pkg/.gitignore containing "*". That silently excluded the
// whole pkg/ directory from the tarball, producing a package whose `main`
// (pkg/suo_parser_wasm.js) does not exist.
//
// Runs from prepack, so a bad package cannot reach npm.

import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const pkgDir = join(root, "pkg");
const entry = join(pkgDir, "suo_parser_wasm.js");
const manifest = JSON.parse(readFileSync(join(root, "package.json"), "utf8"));

const problems = [];

if (existsSync(join(pkgDir, ".gitignore"))) {
  problems.push(
    "pkg/.gitignore still exists - npm would exclude the whole pkg/ directory from the tarball.",
  );
}

if (!existsSync(entry)) {
  problems.push(`${manifest.main} is missing - run \`npm run build\` first.`);
} else if (!existsSync(join(pkgDir, "suo_parser_wasm_bg.wasm"))) {
  problems.push("pkg/suo_parser_wasm_bg.wasm is missing - the package would not load.");
}

// Confirm the entry point actually loads and exposes the public API.
if (existsSync(entry)) {
  try {
    const wasm = createRequire(import.meta.url)(entry);
    const required = ["parse", "transform", "tokenize"];
    const missing = required.filter((name) => typeof wasm[name] !== "function");
    if (missing.length > 0) {
      problems.push(`wasm module is missing exports: ${missing.join(", ")}`);
    } else {
      const roundTrip = wasm.transform('0.0 "Test" sync /re/');
      if (roundTrip !== '0.0 "Test" sync /re/') {
        problems.push(`transform() round-trip returned unexpected output: ${roundTrip}`);
      }
    }
  } catch (error) {
    problems.push(`wasm entry failed to load: ${error.message}`);
  }
}

if (problems.length > 0) {
  console.error("Refusing to pack. Fix the following first:");
  for (const problem of problems) {
    console.error(`  - ${problem}`);
  }
  process.exit(1);
}

console.log("pkg verified: wasm entry loads and round-trips.");