# suo-parser

`suo-parser` is a parser for parsing timeline files that are used in [ACT Timeline](https://github.com/grindingcoil/act_timeline),
[FairyZeta's ACT.Timeline](https://github.com/FairyZeta/ACT.Timeline), [cactbot](https://github.com/quisquous/cactbot), etc.

## Timeline Grammar

This project is planed to support the following grammar:

```text
timeline = { entry | hide-all-stmt | alert-all-stmt | define-stmt | text-popup-stmt }

entry = time ws name [ws sync-stmt] [ws duration-stmt] [ws window-stmt] [ws jump-stmt]
hide-all-stmt = "hideall" ws name
alert-all-stmt = "alertall" ws name [ws before-stmt] [ws sound-stmt]
define-stmt = "define" ws "alertsound" ws name ws file-name
text-popup-stmt = ( "info" | "alert" | "alarm" ) "text" ws name time before-stmt [ws name]

sync-stmt = "sync" ws "/" regex "/"
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
