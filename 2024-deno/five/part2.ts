const instructions = Deno.readTextFileSync("./sample");

const rules: number[][] = [];
const updates: number[][] = [];
instructions.split("\n").forEach((line) => {
  if (!line) return;
  line.includes("|")
    ? rules.push(line.split("|").map((val) => parseInt(val, 10)))
    : updates.push(line.split(",").map((val) => parseInt(val, 10)));
});

const total = updates.map((update) => {
  const failedRules = rules.filter((rule) => {
    return update.includes(rule[0]) && update.includes(rule[1])
      ? update.indexOf(rule[0]) > update.indexOf(rule[1])
      : false;
  });

  if (!failedRules.length) {
    return 0;
  }

  console.log()
  console.log({update})
  console.log({failedRules})

  const repairedUpdate = [...update]
  failedRules.forEach(([first, second]) => {
    const firstIndex = repairedUpdate.indexOf(first);
    const secondIndex = repairedUpdate.indexOf(second);
    if (firstIndex < secondIndex){
      return;
    }

    const firstVal = repairedUpdate[firstIndex];
    const secondVal = repairedUpdate[secondIndex];
    repairedUpdate[secondIndex] = firstVal;
    repairedUpdate[firstIndex] = secondVal;
  })
  console.log({repairedUpdate});


  const mid = repairedUpdate[Math.floor(update.length / 2)];
  return mid 

}).reduce((acc, curr) => acc + curr, 0);

console.log({ total });
