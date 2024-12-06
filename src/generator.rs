// import { Entry, Program, Statement } from "./types";

// export interface GeneratorOptions {
//   target?: "cactbot";
// }

// export class Generator {
//   ast: Program;
//   options: Required<GeneratorOptions>;

//   constructor(ast: Program, options: GeneratorOptions = {}) {
//     this.ast = ast;

//     this.options = {
//       target: "cactbot",
//       ...options,
//     };
//   }

//   generate(): string {
//     if (this.ast.type !== "Program" || !this.ast.body) {
//       throw new Error("Invalid AST");
//     }

//     return this.generateBody(this.ast.body);
//   }

//   generateBody(body: Array<Statement | Entry>): string {
//     const ret: string[] = [];
//     for (const stmt of body) {
//       if (stmt.type === "Entry") {
//         ret.push(this.generateEntry(stmt));
//       } else {
//         ret.push(this.generateStatement(stmt));
//       }
//     }
//     return ret.join("\n");
//   }

//   generateEntry(stmt: Entry): string {
//     const time = stmt.time.value.toFixed(1); // time should be a float with 1 digit
//     const name = this.escapeString(stmt.name.value);
//     let ret = `${time} "${name}"`;
//     if (stmt.sync && stmt.sync.type === "SyncStatement") {
//       const regex = stmt.sync.regex.pattern.replace(/\\/g, "\\\\").replace(/(?<!\\)\//g, "\\/");
//       ret += ` sync /${regex}/`;
//     }
//     if (stmt.sync && stmt.sync.type === "NetSyncStatement") {
//       ret += ` ${stmt.sync.syncType} { ${Object.entries(stmt.sync.fields)
//         .map(([k, v]) => k + ": " + (typeof v === "string" ? '"' + v + '"' : v))
//         .join(", ")} }`;
//     }
//     if (stmt.duration) {
//       const duration = this.simplifyNum(stmt.duration.time.value);
//       ret += ` duration ${duration}`;
//     }
//     if (stmt.window) {
//       const before = stmt.window.before.value;
//       const after = stmt.window.after?.value;
//       if (after && before !== after) {
//         ret += ` window ${this.simplifyNum(before)},${this.simplifyNum(after)}`;
//       } else {
//         ret += ` window ${this.simplifyNum(before)}`;
//       }
//     }
//     if (stmt.jump) {
//       const jump = stmt.jump.time.value;
//       ret += ` jump ${this.simplifyNum(jump)}`;
//     }

//     return ret;
//   }

//   generateStatement(stmt: Statement): string {
//     switch (stmt.type) {
//       case "HideAllStatement":
//         const name = this.escapeString(stmt.name.value);
//         return `hideall "${name}"`;
//       default:
//         throw new Error(`Unsupported statement type: ${stmt.type}`);
//     }
//   }

//   escapeString(str: string): string {
//     return str.replace('"', '\\"').replace("\n", "\\n").replace("\r", "\\r").replace("\t", "\\t");
//   }

//   simplifyNum(num: number): string {
//     if (Number.isInteger(num)) {
//       return num.toString();
//     }
//     return num.toFixed(1);
//   }
// }

#[cfg(test)]
mod tests {
// import { expect } from "chai";
// import { generateAsync, parseAsync, transformAsync } from "../src";

// describe("Generator", () => {
//   it("should generate simple timeline entry", async () => {
//     const timelineText = '0.0 "--Reset--" sync / 00:0839:.*is no longer sealed/ duration 5 window 10000 jump 0';
//     const ast = await parseAsync(timelineText);
//     const timeline = await generateAsync(ast);
//     expect(timeline).to.equal(timelineText);
//   });

//   it("should generate hideall statement", async () => {
//     const timelineText = 'hideall "--sync--"';
//     const ast = await parseAsync(timelineText);
//     const timeline = await generateAsync(ast);
//     expect(timeline).to.equal(timelineText);
//   });

//   it("should transform timeline", async () => {
//     const timelineText = '0.0 "--Reset--" sync / 00:0839:.*is no longer sealed/ duration 5 window 10000 jump 0';
//     const timeline = await transformAsync(timelineText);
//     expect(timeline).to.equal(timelineText);
//   });

//   it("should transform timeline with net sync", async () => {
//     const timelineText = '0.0 "name" Ability { id: "1000", name: "name" } window 10';
//     const timeline = await transformAsync(timelineText);
//     expect(timeline).to.equal(timelineText);
//   });

//   it("should transform hideall statement", async () => {
//     const timelineText = 'hideall "--sync--"';
//     const timeline = await transformAsync(timelineText);
//     expect(timeline).to.equal(timelineText);
//   });
// });

}
