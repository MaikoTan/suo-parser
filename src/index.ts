import { readFile } from "fs";
import { Generator, GeneratorOptions } from "./generator";
import { Parser } from "./parser";
import { Tokenizer } from "./tokenizer";
import { Program } from "./types";

export function parse(code: string, callback: (err: Error | null, program: Program | null) => void): void {
  const tokenizer = new Tokenizer(code);
  const parser = new Parser(tokenizer);
  try {
    const program = parser.parse();
    callback(null, program);
  } catch (err) {
    callback(err as Error, null);
  }
}

export function parseAsync(code: string): PromiseLike<Program> {
  return new Promise<Program>((resolve, reject) => {
    parse(code, (err, program) => {
      if (err || !program) {
        reject(err);
      } else {
        resolve(program);
      }
    });
  });
}

export function parseFile(filePath: string, callback: (err: Error | null, program: Program | null) => void): void {
  readFile(filePath, "utf8", (err, code) => {
    if (err) {
      callback(err, null);
    }
    parse(code, callback);
  });
}

export function parseFileAsync(filePath: string): PromiseLike<Program> {
  return new Promise<Program>((resolve, reject) => {
    parseFile(filePath, (err, program) => {
      if (err || !program) {
        reject(err);
      } else {
        resolve(program);
      }
    });
  });
}

export function generate(ast: Program, callback: (err: Error | null, output: string) => void): void;
export function generate(
  ast: Program,
  options: GeneratorOptions,
  callback: (err: Error | null, output: string) => void,
): void;
export function generate(
  ast: Program,
  optionsOrCallback: GeneratorOptions | ((err: Error | null, output: string) => void),
  callback?: (err: Error | null, output: string) => void,
): void {
  let options = {};
  if (typeof optionsOrCallback === "function") {
    callback = optionsOrCallback;
    options = {};
  }
  callback?.(null, new Generator(ast, options).generate());
}

export function generateAsync(ast: Program, options?: GeneratorOptions): PromiseLike<string> {
  return new Promise<string>((resolve, reject) => {
    generate(ast, options ?? {}, (err, output) => {
      if (err) {
        reject(err);
      } else {
        resolve(output);
      }
    });
  });
}
