const data = await Deno.readTextFile("./input");

console.log(
  data.split("\n").filter((report) => {
    const levels = report.split(" ").map((val) => parseInt(val, 10));

    if(!levels.length) return false;

    let curr = levels[0];
    let currIndex = 1; 
    const direction = typeof curr === "number" && curr > levels[1] ? "DOWN" : "UP"

    while (levels.length > currIndex){
      const next = levels[currIndex];

      if(typeof next !== "number" || typeof curr !== "number") {
        return false
      };

      const diff = Math.abs(curr - next);
      const isInRange = diff === 1 || diff === 2 || diff === 3
      const isMovingCorrectWay = direction === "DOWN" && curr > next || direction === "UP" && curr < next 
      if(isInRange && isMovingCorrectWay){
        currIndex++
        curr = next;
      }else{
        return false
      }
    }

    return true

  }).length
);