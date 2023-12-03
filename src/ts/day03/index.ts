import { getSumOf, getProductOf, readInput, isDigit } from "../utils";

const input = await readInput();
type Point = {
  x: number;
  y: number;
};
type AdjacencyInfo = {
  point: Point;
  isAdjacent: boolean;
};

const DEFAULT_POINT = { y: 0, x: 0 };
console.log({ part1: part1(), part2: part2() });

function processEngine(engine: string[][], predicate: (ch: string) => boolean) {
  const symbolAdjacencyInfo = {
    isAdjacent: false,
    point: DEFAULT_POINT,
  };
  let digit = "";

  const result = engine.flatMap((line, rowIndex) =>
    line.reduce<{ digit: number; point: AdjacencyInfo["point"] }[]>(
      (acc, char, colIndex) => {
        if (isSymbol(char) || char === ".") {
          if (symbolAdjacencyInfo.isAdjacent) {
            acc.push({
              digit: Number(digit),
              point: { ...symbolAdjacencyInfo.point },
            });
          }
          digit = "";
          Object.assign(symbolAdjacencyInfo, {
            isAdjacent: false,
            point: {},
          });
        } else if (isDigit(char)) {
          const adjacency = getSymbolAdjacencyInfo({
            row: rowIndex,
            col: colIndex,
            engine,
            predicate,
          });

          if (adjacency.isAdjacent) {
            Object.assign(symbolAdjacencyInfo, adjacency);
          }
          digit += char;
        }

        return acc;
      },
      [],
    ),
  );

  return result;
}

function part1() {
  const engine = input.map((x) => x.split(""));
  const result = processEngine(engine, isSymbol);
  return getSumOf(result.map((x) => x.digit));
}

function part2() {
  const engine = input.map((x) => x.split(""));
  const result = processEngine(engine, isGear);

  const products = result.flatMap((first, index) => {
    return result.slice(index + 1).flatMap((second) => {
      if (isClose(first.point, second.point)) {
        return getProductOf([first.digit, second.digit]);
      }

      return 0;
    });
  });

  return getSumOf(products.filter(Boolean));
}

function isClose(a: AdjacencyInfo["point"], b: AdjacencyInfo["point"]) {
  return Math.abs(a.y - b.y) <= 2 && Math.abs(a.x - b.x) <= 2;
}

function getSymbolAdjacencyInfo({
  row,
  col,
  engine,
  predicate,
}: {
  row: number;
  col: number;
  engine: string[][];
  predicate: (ch: string) => boolean;
}): AdjacencyInfo {
  const validCoordinates = getValidCoordinates(
    getOffsets(row, col),
    engine.length,
  );

  const adjacentCoordinate = validCoordinates.find(([x, y]) =>
    predicate(engine[y][x]),
  );

  const point = adjacentCoordinate
    ? { y: adjacentCoordinate[1] + row, x: adjacentCoordinate[0] + col }
    : DEFAULT_POINT;

  return {
    isAdjacent: Boolean(adjacentCoordinate),
    point,
  };
}

function getOffsets(row: number, col: number) {
  return [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1],
    [-1, -1],
    [1, -1],
    [-1, 1],
    [1, 1],
  ].map(([x, y]) => [x + col, y + row]);
}

function getValidCoordinates(offsets: number[][], size: number) {
  return offsets.filter(([x, y]) => x >= 0 && x < size && y >= 0 && y < size);
}

function isSymbol(ch: string): boolean {
  return /[^\w.]/.test(ch);
}

function isGear(ch: string): boolean {
  return ch === "*";
}
