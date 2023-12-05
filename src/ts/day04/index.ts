import { readInput } from "../utils";

const input = await readInput();

type Card = {
  index: number;
  matches: number;
};

console.log({ part1: part1(), part2: part2() });

function part1() {
  const cards = getCards(input);

  return cards.reduce((acc, card) => {
    if (!card.matches) return acc;
    return acc + Math.pow(2, card.matches - 1);
  }, 0);
}

function part2() {
  const cards = getCards(input);
  const copiesOfScratchCards = [];

  for (const card of cards) {
    copiesOfScratchCards.push(card);

    for (const copy of copiesOfScratchCards) {
      if (copy.index !== card.index) continue;

      copiesOfScratchCards.push(
        ...cards.slice(copy.index + 1, copy.index + 1 + copy.matches),
      );
    }
  }

  return copiesOfScratchCards.length;
}

function getCards(input: string[]): Card[] {
  return input.map((line, index) => {
    const [, numbers] = line.split(": ");
    const [winningNumbers, scratchCards] = numbers
      .split(" | ")
      .map((nums) => nums.split(" ").map(Number).filter(Boolean));

    const matches = scratchCards.filter((num) =>
      winningNumbers.includes(num),
    ).length;

    return { matches, index };
  });
}
