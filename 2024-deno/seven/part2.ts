// const data = await Deno.readTextFile("./sample");
const data = await Deno.readTextFile("./input");

const instructions = ["+", "*", "|"];
const add = (a: number, b: number) => a + b;
const multiply = (a: number, b: number) => a * b;
const smoosh = (a: number, b: number) => parseInt(String(a) + String(b), 10);

function executeFormula(formula: (string | number)[]): number {
  const working = [...formula];
  while (working.length > 1) {

    const workingIndex = working.findIndex(val => typeof val === "string");
    const parts = [working[workingIndex - 1], working[workingIndex + 1]];
    if (typeof parts[0] === "string" || typeof parts[1] === "string") {
      console.error("Invalid formula", parts);
      Deno.exit(1);
    }

    const operator = working[workingIndex] === "+" 
    ? add
    : working[workingIndex] === "*" ? multiply : smoosh;

    const value = operator(parts[0], parts[1]);
    working.splice(workingIndex - 1, 3, value);
  }

  return working[0] as number;
}

function genCombinations(arr: string[], min = 1): string[] {
  const max = min;
  const combination = (arr: string[], depth: number): string[] => {
    if (depth === 1) {
      return arr;
    } else {
      const result = combination(arr, depth - 1).flatMap((val: string) =>
        arr.map((char: string) => val + char)
      );
      return arr.concat(result);
    }
  };

  return combination(arr, max).filter((val: string) => val.length >= min);
}

const total = data.split("\n").map((equation) => {
  if (!equation?.length) {
    return;
  }

  const [resultStr, valsStr] = equation.split(":");
  const result = parseInt(resultStr, 10);
  const vals = valsStr.trim().split(" ").map((val) => parseInt(val, 10));
  const variations = genCombinations(instructions, vals.length - 1);
  console.log({equation});

  const anyPasses = variations.filter((variation) => {
    const operations = variation.split("");
    const formula = vals.flatMap((val, index) => [val, operations[index]])
      .filter(Boolean);

    const executionValue = executeFormula(formula)
    // console.log({result, executionValue, variation})
    return result ===  executionValue
  });
  // console.log({ anyPasses });

  return anyPasses.length ? result  : undefined;
}).filter((val) => typeof val === "number").reduce(
  (cur, acc: number) => cur ? cur + acc : acc,
  0,
);

console.log({ total });
