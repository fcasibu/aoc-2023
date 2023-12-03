import { getSumOf, getProductOf, readInput } from "../utils";

const input = await readInput();
const config = {
  red: 12,
  green: 13,
  blue: 14,
} as const;

type RGB = keyof typeof config;

console.log({ part1: part1(), part2: part2() });

function part1() {
  const gameIds = input.map((line, index) => {
    const gameResult = getSets(line).filter(checkIfPossibleToPlay).length;
    return gameResult == 0 ? index + 1 : 0;
  });

  return getSumOf(gameIds);
}

function part2() {
  const products = input.map((line) => {
    const gameResult = getSets(line).map(countCubes);
    return getProductOf(getMaxValuesOf(gameResult));
  });

  return getSumOf(products);
}

function getSets(line: string): string[] {
  return line.split(":")[1].split(";");
}

function checkIfPossibleToPlay(set: string): boolean {
  const { red, blue, green } = countCubes(set);
  return red > config.red || blue > config.blue || green > config.green;
}

function countCubes(set: string): Record<RGB, number> {
  const colorPatterns = {
    red: /(\d+)\sred/g,
    blue: /(\d+)\sblue/g,
    green: /(\d+)\sgreen/g,
  };

  const entries = Object.entries(colorPatterns) as [RGB, RegExp][];

  return entries.reduce(
    (acc, [color, pattern]) => {
      const cubes = set.match(pattern)?.map((str) => parseInt(str, 10));
      acc[color] = getSumOf(cubes ?? []);
      return acc;
    },
    { red: 0, blue: 0, green: 0 },
  );
}

function getMaxValuesOf(game: Record<RGB, number>[]): number[] {
  const maxValues = game.reduce(
    (acc, color) => {
      acc.red = Math.max(acc.red, color.red);
      acc.blue = Math.max(acc.blue, color.blue);
      acc.green = Math.max(acc.green, color.green);
      return acc;
    },
    { red: 0, green: 0, blue: 0 },
  );

  return Object.values(maxValues);
}
