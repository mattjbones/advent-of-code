// const diskMap = Deno.readTextFileSync("./sample");
const diskMap = Deno.readTextFileSync("./input");

function generateBlockMap(diskMap: string): string[] {
  const blockMap = [];
  let isFile = true;
  let fileId = 0;
  for (const val of diskMap) {
    if (isFile) {
      blockMap.push(new Array(parseInt(val)).fill(`${fileId}`));
      fileId++;
      isFile = false;
    } else {
      blockMap.push(new Array(parseInt(val)).fill("."));
      isFile = true;
    }
  }
  return blockMap.flat();
}

function hasGaps(blockMap: string[]): boolean {
  let hasGaps = false;
  let isDot = false;
  for (const el of blockMap) {
    if (el === ".") {
      isDot = true;
    }
    if (el !== "." && isDot) {
      hasGaps = true;
      break;
    }
  }

  return hasGaps;
}

function moveFileBlock(
  blockMap: string[],
): string[] {
  const moved = [...blockMap];
  const indexOfGap = moved.indexOf(".");
  const nonDotToMoveIndex = moved.length - 1 -
    moved.toReversed().findIndex((val) => val !== ".");
  const valToMove = moved[nonDotToMoveIndex];
  moved.splice(indexOfGap, 1, valToMove);
  moved[nonDotToMoveIndex] = ".";
  return moved;
}

function calculateTotal(blockMap: string[]): number {
  return blockMap.reduce(
    (acc, val, index) => val !== "." ? acc + (parseInt(val, 10) * index) : acc,
    0,
  );
}

console.log({ diskMap });
const blockMap = generateBlockMap(diskMap);
console.log(blockMap.join(""));

let workingBlockMap = [...blockMap];
while (hasGaps(workingBlockMap)) {
  workingBlockMap = moveFileBlock(workingBlockMap);
}

console.log({ workingBlockMap });
console.log({ total: calculateTotal(workingBlockMap) });
