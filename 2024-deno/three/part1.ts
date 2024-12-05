const instruction = Deno.readTextFileSync("./input");
const regy = new RegExp(/mul\((\d*),(\d*)\)/, "g");
let [sum, valA, valB, ] = regy.exec(instruction) || []
let working = `${instruction}`;
let total = 0;
while (sum){
  const result = parseInt(valA, 10) * parseInt(valB, 10);
  total += result
  working.replace(sum, `${result}`);
  [sum, valA, valB] = regy.exec(instruction) || []
}

console.log({total});
