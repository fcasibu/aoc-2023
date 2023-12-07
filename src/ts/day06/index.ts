import { getProductOf, readInput, zip } from "../utils";

const input = await readInput("\n");

console.log({ part1: part1(), part2: part2() });

type Race = {
  time: number;
  distance: number;
};

function part1() {
  return getNumOfWays(getTimeAndDistance(input));
}

function part2() {
  return getNumOfWays(getTimeAndDistance2(input));
}

function getNumOfWays(races: Race[]): number {
  return getProductOf(
    races.map(
      (race) =>
        Array.from({ length: race.time }, (_, i) => i).filter(
          (i) => i * (race.time - i) > race.distance,
        ).length,
    ),
  );
}

function getTimeAndDistance(input: string[]): Race[] {
  const times = Array.from(input[0].matchAll(/\d+/g), Number);
  const distances = Array.from(input[1].matchAll(/\d+/g), Number);
  return zip(times, distances).map(([time, distance]) => ({
    time,
    distance,
  }));
}

function getTimeAndDistance2(input: string[]): Race[] {
  const time = Number(input[0].replaceAll(/\D/g, ""));
  const distance = Number(input[1].replaceAll(/\D/g, ""));
  return [{ time, distance }];
}
