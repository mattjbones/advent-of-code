const instruction = Deno.readTextFileSync("./input");

const regy = new RegExp(/mul\((\d*),(\d*)\)/, "g");
const match = regy.exec(instruction) 
if(match === null){
  Deno.exit(1);
}
let [sum, valA, valB] = match;
let index = match["index"] 
let working = `${instruction}`;
let total = 0;
while (true) {
  const disabled = new RegExp(/don't\(\)/, "g");
  const disabledMatches = disabled.exec(working);
  const disabledStart = disabledMatches !== null ? disabledMatches["index"] : Infinity;
  const canUseSum = index < disabledStart;
  if (canUseSum){
    total+= parseInt(valA, 10) * parseInt(valB, 10);
    working = working.replace(sum, `${parseInt(valA, 10) * parseInt(valB, 10)}`);
  }else{
    const enabled = new RegExp(/do\(\)/, "g");
    const enabledMatch = enabled.exec(working);
    if(!enabledMatch) break;
    const enabledStart = enabledMatch["index"];
    working = working.substring(enabledStart + "do()".length)
  }

  const regy = new RegExp(/mul\((\d*),(\d*)\)/, "g");
  const nextMatch = regy.exec(working);
  if(!nextMatch) break;
  
  [sum, valA, valB] = nextMatch;
  index = nextMatch["index"] 
}
console.log({total});
