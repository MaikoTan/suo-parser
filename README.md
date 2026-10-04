# 梭 suo-parser

![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/MaikoTan/suo-parser/test.yml?style=for-the-badge&label=test) [![npm](https://img.shields.io/npm/v/suo-parser?style=for-the-badge)](https://www.npmjs.com/package/suo-parser)

`梭 (suo-parser)` is a parser for parsing timeline files that are used in [ACT Timeline](https://github.com/grindingcoil/act_timeline),
[FairyZeta's ACT.Timeline](https://github.com/FairyZeta/ACT.Timeline), [cactbot](https://github.com/OverlayPlugin/cactbot), etc.

> `梭` (pronounced /swo̞˥/) in Chinese means "shuttle" (a device that is used in weaving to carry the thread),
> therefore, this word is also extended in meaning as "fast" in Chinese.
>
> `梭 (suo-parser)` wants to be a fast and accurate parser for ACT Timeline files.

## Features

* Parse a timeline file into an AST.

  > The AST types are defined in [`core/src/types.rs`](core/src/types.rs).

* Transform a timeline file to a specific format (currently, only [cactbot](https://github.com/OverlayPlugin/cactbot/blob/main/docs/TimelineGuide.md#timeline-file-syntax) style is supported).

* Tokenize a timeline file, for debugging.

## Install

```bash
# If you use npm:
$ npm install suo-parser
# If you use yarn:
$ yarn add suo-parser
```

## Usage

### Command line

Round-trip a timeline file through the parser and generator:

```bash
$ suo-parser timeline.txt
0.0 "--Reset--" sync / 00:0839:.*is no longer sealed/ duration 5 window 10000 jump 0
```

Print the AST as JSON instead:

```bash
$ suo-parser --ast timeline.txt
```

Read from stdin by omitting the file or passing `-`:

```bash
$ cat timeline.txt | suo-parser
```

> If you are using yarn, you can run `yarn suo-parser <filename>` instead.

### Library

The package ships a WebAssembly build. All functions are synchronous.

```js
const { parse, transform, generate, tokenize, version } = require("suo-parser")

// Parse into an AST: { defines, hide_alls, alert_alls, entries }
const ast = parse('0.0 "Test" sync /re/')
ast.entries[0].name // => "Test"

// Round-trip a timeline back to text
transform('0.0 "Test" sync /re/') // => '0.0 "Test" sync /re/'

// Generate text from an AST
generate(ast)

// Inspect the token stream (useful when a timeline fails to parse)
tokenize('0.0 "Test"')

version() // => "0.3.0-rc.1"
```

Times are a tagged union, because the grammar allows both integers and floats:

```js
ast.entries[0].time // => { Float: 0 }  or  { Integer: 0 }
```

`parse` throws on malformed input. The wasm bindings install a panic hook, so
the underlying message is printed to stderr before the throw.

## Migrating from 0.2.x

> **Note**: `0.3.0` is currently a release candidate. Install it with
> `npm install suo-parser@next`, since prereleases are published under the
> `next` dist-tag rather than `latest`.

`0.3.0` replaces the TypeScript + napi-rs implementation with a Rust +
WebAssembly one, and the API is different:

| 0.2.x                                  | 0.3.0                                    |
| -------------------------------------- | ---------------------------------------- |
| `parse(code, callback)`                | `parse(code)` — synchronous             |
| `parseAsync(code)`                     | removed — `parse` is already synchronous |
| `parseFile` / `parseFileAsync`         | removed — read the file yourself         |
| `generate(ast, callback)`              | `generate(ast)` — synchronous           |
| `generateAsync(ast)`                   | removed — `generate` is synchronous     |
| `transformAsync(code)`                 | removed — `transform` is synchronous    |
| `transformFile` / `transformFileAsync` | removed — read the file yourself         |
| `suo` binary                           | renamed to `suo-parser`                 |

The AST shape also changed; it is now a plain object with `defines`,
`hide_alls`, `alert_alls` and `entries` keys.

## Supporting Timeline Grammar

This project is planed to support the following grammar:

(It is not fully implemented yet. sorry!)

```text
timeline = { entry | hide-all-stmt | alert-all-stmt | define-stmt | text-popup-stmt }

entry = time ws name [ws sync-stmt] [ws duration-stmt] [ws window-stmt] [ws jump-stmt]
hide-all-stmt = "hideall" ws name
alert-all-stmt = "alertall" ws name [ws before-stmt] [ws sound-stmt]
define-stmt = "define" ws "alertsound" ws name ws file-name
text-popup-stmt = ( "info" | "alert" | "alarm" ) "text" ws name time before-stmt [ws name]

sync-stmt = sync-regex-stmt | sync-netsync-stmt
sync-regex-stmt = "sync" ws "/" regex "/"
sync-netsync-stmt = netsync-type ws "{" ws sync-netsync-key-value-pair (ws "," ws sync-netsync-key-value-pair)* ws "}"
netsync-type = STRING+
sync-netsync-key-value-pair = name ws ":" ws '"' STRING+ '"'

duration-stmt = "duration" ws time
window-stmt = "window" ws time [ [ws] "," [ws] time ]
jump-stmt = "jump" ws time
before-stmt = "before" ws time
sound-stmt = "sound" ws file-name

file-name = name
name = '"' STRING+ '"' | STRING+
time = NUMBER+ | NUMBER+ "." NUMBER+
regex = ? regular expression literal ?
ws = ( " " | "\t" )*
```
