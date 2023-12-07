import { join } from "path";

export async function readInput(separator: string) {
  const { argv, cwd, exit } = process;
  const file_path = argv[2];

  if (!file_path) throw new Error("Provide a file path to input");

  try {
    const file = await Bun.file(join(cwd(), file_path)).text();

    return file.trim().split(separator);
  } catch (e) {
    console.log("Something went wrong with reading file", e);
    return exit(1);
  }
}

export function getSumOf(nums: number[]): number {
  return nums.reduce((acc, curr) => acc + curr, 0);
}

export function getProductOf(nums: number[]): number {
  return nums.reduce((acc, curr) => acc * curr, 1);
}

export function isDigit(ch: string): boolean {
  return ch >= "0" && ch <= "9";
}

export function isLetter(ch: string): boolean {
  return (ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z");
}

export function zip<T extends unknown[], U extends unknown[]>(
  a: T,
  b: U,
): [T[number], U[number]][] {
  return a.map((val, i) => [val, b[i % b.length]]);
}
