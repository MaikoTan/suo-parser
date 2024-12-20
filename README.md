# 梭 suo-parser

![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/MaikoTan/suo-parser/test.yml?style=for-the-badge&label=test) [![npm](https://img.shields.io/npm/v/suo-parser?style=for-the-badge)](https://www.npmjs.com/package/suo-parser)

`梭 (suo-parser)` is a parser for parsing timeline files that are used in [ACT Timeline](https://github.com/grindingcoil/act_timeline),
[FairyZeta's ACT.Timeline](https://github.com/FairyZeta/ACT.Timeline), [cactbot](https://github.com/OverlayPlugin/cactbot), etc.

> `梭` (pronounced /swo̞˥/) in Chinese means "shuttle" (a device that is used in weaving to carry the thread),
> therefore, this word is also extended in meaning as "fast" in Chinese.
>
> `梭 (suo-parser)` wants to be a fast and accurate parser for ACT Timeline files.

## Features

* Parse a timeline file into AST that is compatible with [ESTree](https://github.com/estree/estree).

  > The AST types are defined in [src/types.ts](src/types.ts).

* Transform a timeline file to a specific format (currently, only [cactbot](https://github.com/OverlayPlugin/cactbot/blob/main/docs/TimelineGuide.md#timeline-file-syntax) style is supported).

## Install

```bash
# If you use npm:
$ npm install suo-parser
# If you use yarn:
$ yarn add suo-parser
```

And then you can run `suo <filename>` to transform a timeline file to a specifical format.

> If you are using yarn, you can run `yarn suo <filename>` instead.

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
