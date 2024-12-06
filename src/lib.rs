mod generator;
mod parser;
mod tokenizer;
mod types;
mod utils;

// import { readFile } from "fs";
// import { Generator, GeneratorOptions } from "./generator";
// import { Parser } from "./parser";
// import { Tokenizer } from "./tokenizer";
// import { Program } from "./types";

// export function parse(code: string, callback: (err?: Error, program?: Program) => void): void {
//   const tokenizer = new Tokenizer(code);
//   const parser = new Parser(tokenizer);
//   try {
//     const program = parser.parse();
//     callback(undefined, program);
//   } catch (err) {
//     callback(err as Error);
//   }
// }

// export function parseAsync(code: string): PromiseLike<Program> {
//   return new Promise<Program>((resolve, reject) => {
//     parse(code, (err, program) => {
//       if (err || !program) {
//         reject(err);
//       } else {
//         resolve(program);
//       }
//     });
//   });
// }

// export function parseFile(filePath: string, callback: (err?: Error, program?: Program) => void): void {
//   readFile(filePath, "utf8", (err, code) => {
//     if (err) {
//       callback(err);
//     }
//     parse(code, callback);
//   });
// }

// export function parseFileAsync(filePath: string): PromiseLike<Program> {
//   return new Promise<Program>((resolve, reject) => {
//     parseFile(filePath, (err, program) => {
//       if (err || !program) {
//         reject(err);
//       } else {
//         resolve(program);
//       }
//     });
//   });
// }

// export function generate(ast: Program, callback: (err?: Error, output?: string) => void): void;
// export function generate(
//   ast: Program,
//   options: GeneratorOptions,
//   callback: (err?: Error, output?: string) => void,
// ): void;
// export function generate(
//   ast: Program,
//   optionsOrCallback: GeneratorOptions | ((err?: Error, output?: string) => void),
//   callback?: (err?: Error, output?: string) => void,
// ): void {
//   let options = {};
//   if (typeof optionsOrCallback === "function") {
//     callback = optionsOrCallback;
//     options = {};
//   }
//   callback?.(undefined, new Generator(ast, options).generate());
// }

// export function generateAsync(ast: Program, options?: GeneratorOptions): PromiseLike<string> {
//   return new Promise<string>((resolve, reject) => {
//     generate(ast, options ?? {}, (err, output) => {
//       if (err || !output) {
//         reject(err);
//       } else {
//         resolve(output);
//       }
//     });
//   });
// }

// export type TransformOptions = GeneratorOptions;

// export function transform(code: string, callback: (err?: Error, output?: string) => void): void;
// export function transform(
//   code: string,
//   options: TransformOptions,
//   callback: (err?: Error, output?: string) => void,
// ): void;
// export function transform(
//   code: string,
//   optionsOrCallback: TransformOptions | ((err?: Error, output?: string) => void),
//   callback?: (err?: Error, output?: string) => void,
// ): void {
//   let options = {};
//   if (typeof optionsOrCallback === "function") {
//     callback = optionsOrCallback;
//     options = {};
//   }
//   parse(code, (err, ast) => {
//     if (err || !ast) {
//       callback?.(err);
//     } else {
//       generate(ast, options, (err, output) => {
//         if (err) {
//           callback?.(err);
//         } else {
//           callback?.(undefined, output);
//         }
//       });
//     }
//   });
// }

// export function transformAsync(code: string, options?: GeneratorOptions): PromiseLike<string> {
//   return new Promise<string>((resolve, reject) => {
//     transform(code, options ?? {}, (err, output) => {
//       if (err || !output) {
//         reject(err);
//       } else {
//         resolve(output);
//       }
//     });
//   });
// }

// export function transformFile(filePath: string, callback: (err?: Error, output?: string) => void): void;
// export function transformFile(
//   filePath: string,
//   options: TransformOptions,
//   callback: (err?: Error, output?: string) => void,
// ): void;
// export function transformFile(
//   filePath: string,
//   optionsOrCallback: TransformOptions | ((err?: Error, output?: string) => void),
//   callback?: (err?: Error, output?: string) => void,
// ): void {
//   let options = {};
//   if (typeof optionsOrCallback === "function") {
//     callback = optionsOrCallback;
//     options = {};
//   }
//   if (typeof callback === "undefined") {
//     throw new Error("callback is required");
//   }
//   readFile(filePath, "utf8", (err, code) => {
//     if (err) {
//       callback?.(err);
//     } else {
//       transform(code, options, (err, output) => {
//         if (err) {
//           callback?.(err);
//         } else {
//           callback?.(undefined, output);
//         }
//       });
//     }
//   });
// }

// export function transformFileAsync(filePath: string, options?: TransformOptions): PromiseLike<string> {
//   return new Promise<string>((resolve, reject) => {
//     transformFile(filePath, options ?? {}, (err, output) => {
//       if (err || !output) {
//         reject(err);
//       } else {
//         resolve(output);
//       }
//     });
//   });
// }

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
