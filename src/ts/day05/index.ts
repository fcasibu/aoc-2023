import { readInput } from "../utils";

const input = await readInput("\n\n");

console.log({ part1: part1(), part2: part2() });

type Conversion = {
  sourceRangeStart: number;
  destinationRangeStart: number;
  rangeLength: number;
};

function part1() {
  return findLowestLocation(getSeedNumbers(input), getMappings(input));
}

function part2() {
  const seedNumbers = getSeedNumbers(input);
  let seeds: number[] = [];

  for (let i = 0; i < seedNumbers.length; i += 2) {
    const start = seedNumbers[i];
    const length = seedNumbers[i + 1];
    seeds = seeds.concat(Array.from({ length }, (_, j) => j + start));
  }

  return findLowestLocation(seeds, getMappings(input));
}

function getSeedNumbers(almanac: string[]) {
  return Array.from(almanac[0].matchAll(/\d+/g), Number);
}

function getMappings(almanac: string[]) {
  return almanac
    .slice(1)
    .map((line) => line.split("\n").slice(1).map(convertToRanges));
}

function convertToRanges(mapping: string) {
  const [destinationRangeStart, sourceRangeStart, rangeLength] = Array.from(
    mapping.matchAll(/\d+/g),
    Number,
  );

  return {
    destinationRangeStart,
    sourceRangeStart,
    rangeLength,
  };
}

function convertNumber(sourceNumber: number, conversionMaps: Conversion[]) {
  for (const conversion of conversionMaps) {
    const { destinationRangeStart, sourceRangeStart, rangeLength } = conversion;
    if (
      sourceNumber >= sourceRangeStart &&
      sourceNumber < sourceRangeStart + rangeLength
    ) {
      return destinationRangeStart + (sourceNumber - sourceRangeStart);
    }
  }

  return sourceNumber;
}

// VERY SLOW, only got the answer because I wrote another slow version but in RUST! :lol:
function findLowestLocation(seedNumbers: number[], maps: Conversion[][]) {
  for (const conversions of maps) {
    for (let i = 0; i < seedNumbers.length; ++i) {
      const number = convertNumber(seedNumbers[i], conversions);
      seedNumbers[i] = number;
    }
  }

  return Math.min(...seedNumbers);
}
