const data = await Deno.readTextFile("./input");

console.log(
  data.split("\n").filter((report) => {


    const calc = (levels: number[]): [boolean, number[]]=>{
      const diffs = [];
      for(let i = 0; i < levels.length - 1; i++){
        diffs.push(levels[i] - levels[i+1])
      }
  
      const isIncreasing = diffs.every((val) => val <= 0);
      const isDecreasing = diffs.every((val) => val >= 0);
      const hasValidGap = !diffs.some((val)=> val === 0 || val > 3 || val < -3 );
  
      return [(isIncreasing || isDecreasing) && hasValidGap, diffs]
    }

    const levels = report.split(" ").map((val) => parseInt(val, 10));
    const [safe, ] = calc(levels);
     
    if (safe){
      return true
    }

    for(let i = 0; i< levels.length; i ++){
      const [safe, ] = calc(levels.toSpliced(i,1));

      if(safe){
        return true
      }
    }

    return false;

  }).length,
);
