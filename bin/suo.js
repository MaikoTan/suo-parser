#!/usr/bin/env node
/* eslint-disable */

'use strict'

const { readFileSync } = require("node:fs")
const { parse, transform } = require("../pkg/suo_parser_wasm.js")

function usage(stream) {
  stream.write(
    "Usage: suo-parser [--ast] [FILE]\n" +
      "  Parse a timeline file and either round-trip it (default) or print its AST.\n" +
      "  FILE   timeline file to read, or `-`/omit for stdin\n" +
      "  --ast  print the parsed AST as JSON instead of generating\n"
  )
}

function main() {
  const args = process.argv.slice(2)
  let printAst = false
  let file = null

  for (let i = 0; i < args.length; i++) {
    const a = args[i]
    if (a === "-h" || a === "--help") {
      usage(process.stdout)
      process.exit(0)
    } else if (a === "--ast") {
      printAst = true
    } else {
      file = a
    }
  }

  let input
  try {
    input =
      file === null || file === "-"
        ? readFileSync(0, "utf8")
        : readFileSync(file, "utf8")
  } catch (err) {
    process.stderr.write("error: failed to read input: " + err.message + "\n")
    process.exit(1)
  }

  if (printAst) {
    process.stdout.write(JSON.stringify(parse(input), null, 2) + "\n")
  } else {
    process.stdout.write(transform(input))
  }
}

main()
