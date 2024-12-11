const instructions = Deno.readTextFileSync("./input");

const rules: number[][] = [];
const updates: number[][] = [];
instructions.split("\n").forEach((line) => {
  if (!line) return;
  line.includes("|")
    ? rules.push(line.split("|").map((val) => parseInt(val, 10)))
    : updates.push(line.split(",").map((val) => parseInt(val, 10)));
});

const total = updates.map((update) => {
  const passedAllRules = rules.every((rule) => {
    return update.includes(rule[0]) && update.includes(rule[1])
      ? update.indexOf(rule[0]) < update.indexOf(rule[1])
      : true;
  });

  if (!passedAllRules) {
    return 0;
  }
  const mid = update[Math.floor(update.length / 2)];
  return mid 

}).reduce((acc, curr) => acc + curr, 0);

console.log({ total });
