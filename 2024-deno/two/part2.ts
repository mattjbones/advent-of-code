const data = await Deno.readTextFile("./input");

function checkSafety(levels: Array<number>): [boolean, Array<number>] {

    let current = 0;
    const failed = []
    while(current < levels.length - 1){
      const now = levels[current]
      const next = levels[current + 1];

      const diff = Math.abs(now - next);
      const isRightDiff = diff === 1 || diff === 2 || diff === 3;
      const isRightDir = levels[0] > levels[1] && now > next || levels[0] < levels[1] && now < next;

      if(!isRightDiff || !isRightDir){
        failed.push(current);
      }
      current++;
    }

    return [failed.length === 0, failed]
}

console.log(
  data.split("\n").filter((report) => {
    const levels = report.split(" ").map((val) => parseInt(val, 10));

    if(!levels.length) return false;

    const [safe, failures ] = checkSafety(levels);

    const safeWithRemovals = !safe && failures.map((failedIndex)=>checkSafety([...levels].toSpliced(failedIndex, 1))).some(([val,]) => val);


    return safe || safeWithRemovals

  }).length
);