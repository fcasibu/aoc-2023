import { getSumOf, readInput } from "../utils";

const input = await readInput("\n");
console.log({ part1: part1(), part2: part2() });

function part1() {
  const nums = input.map(getDigits);

  return getSumOf(nums);
}

function part2() {
  const nums = input.map((characters) =>
    getDigits(replaceWordsWithDigits(characters)),
  );

  return getSumOf(nums);
}

function getDigits(characters: string): number {
  const digitsStr = characters.replaceAll(/\D/g, "");

  return Number(concatFirstAndLastCharacters(digitsStr)) || 0;
}

function concatFirstAndLastCharacters(characters: string): string {
  return characters[0] + characters[characters.length - 1];
}

function replaceWordsWithDigits(characters: string): string {
  const digitMap: Record<string, string> = {
    one: "o1e",
    two: "t2o",
    three: "t3e",
    four: "f4r",
    five: "f5e",
    six: "s6x",
    seven: "s7n",
    eight: "e8t",
    nine: "n9n",
  };

  return Object.entries(digitMap).reduce(
    (result, [key, value]) => result.replace(new RegExp(key, "g"), value),
    characters,
  );
}
