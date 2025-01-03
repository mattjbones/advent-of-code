// 0 1 10 99 999
// 1 2024 1 0 9 9 2021976

// const data = Deno.readTextFileSync("./example1"); // 1 2024 1 0 9 9 2021976 - 1 blink
// const data = Deno.readTextFileSync("./sample"); // 2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2 - 6 blinks
const data = Deno.readTextFileSync("./input"); //  blinks

const blinks = 75;

const parts = data.split(" ").map((val) => parseInt(val, 10));
console.log(parts.join(" "));

function applyRuleOne(stone: number): [number, boolean] {
  if (stone === 0) {
    return [1, true];
  }

  return [stone, false];
}

function applyRuleTwo(stone: number): [number[], boolean] {
  const stringStone = `${stone}`;
  if (stringStone.length % 2 === 0) {
    const middle = stringStone.length / 2;
    const firstHalf = stringStone.slice(0, middle);
    const secondHalf = stringStone.slice(middle);
    return [[parseInt(firstHalf, 10), parseInt(secondHalf, 10)], true];
  }

  return [[stone], false];
}

function applyRuleThree(stone: number): [number, boolean] {
  return [stone * 2024, true];
}

function blinkStone(stone: number) {
  const [ruleOneStone, hasAppliedRuleOne] = applyRuleOne(stone);
  if (hasAppliedRuleOne) {
    return [ruleOneStone];
  }

  const [ruleTwoStones, hasAppliedRuleTwo] = applyRuleTwo(stone);
  if (hasAppliedRuleTwo) {
    return ruleTwoStones;
  }

  const [ruleThreeStone] = applyRuleThree(stone);
  return [ruleThreeStone];
}

function countStoneBlinks(
  stone: number,
  depth: number,
  cache: { [stone: number]: { [depth: number]: number } },
): number {
  if (
    typeof cache[stone] !== "undefined" &&
    typeof cache[stone][depth] === "number"
  ) return cache[stone][depth];

  const [left, right] = blinkStone(stone);

  if (depth === 1) {
    if (typeof right === "undefined") {
      return 1;
    } else {
      return 2;
    }
  }

  let output = countStoneBlinks(left, depth - 1, cache);
  if (typeof right !== "undefined") {
    output += countStoneBlinks(right, depth - 1, cache);
  }

  if (cache[stone]) {
    cache[stone][depth] = output;
  } else {
    cache[stone] = { [depth]: output };
  }

  return output;
}

let total = 0;
for (const stone of parts) {
  total += countStoneBlinks(stone, blinks, {});
}

console.log(total);
