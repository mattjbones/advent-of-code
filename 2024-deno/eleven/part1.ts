// const data = Deno.readTextFileSync("./example1"); // 1 2024 1 0 9 9 2021976 - 1 blink
// const data = Deno.readTextFileSync("./sample"); // 2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2 - 6 blinks
const data = Deno.readTextFileSync("./input"); // 25 blinks

const blinks = 25;

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

let stones = [...parts];
for (let blink = 0; blink < blinks; ++blink) {
  for (let i = 0; i < stones.length; ++i) {
    const currentStone = stones[i];

    const [ruleOneStone, hasAppliedRuleOne] = applyRuleOne(currentStone);
    if (hasAppliedRuleOne) {
      stones[i] = ruleOneStone;
      continue;
    }

    const [ruleTwoStones, hasAppliedRuleTwo] = applyRuleTwo(currentStone);
    if (hasAppliedRuleTwo) {
      const start = stones.slice(0, i);
      const end = stones.slice(i + 1);
      stones = [...start, ...ruleTwoStones, ...end];
      i += 1;
      continue;
    }

    const [ruleThreeStone] = applyRuleThree(currentStone);
    stones[i] = ruleThreeStone;
  }
  // console.log(stones.join(" "));
  console.log("blink: ", blink);
}

console.log(stones.length);
