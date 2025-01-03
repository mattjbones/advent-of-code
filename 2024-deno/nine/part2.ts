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

function findBlock(blockMap: string[], startIndex: number, freeSpace = true) {
  let nextIndex = -1;
  for (let i = startIndex; i < blockMap.length; ++i) {
    const el = blockMap[i];
    if (freeSpace ? el === "." : el !== ".") {
      nextIndex = i;
      break;
    }
  }
  return nextIndex;
}

function findBlockReversed(
  blockMap: string[],
  startIndex: number,
  freeSpace = true,
) {
  let nextIndex = -1;
  for (let i = startIndex; i >= 0; i--) {
    const el = blockMap[i];
    if (freeSpace ? el === "." : el !== ".") {
      nextIndex = i;
      break;
    }
  }
  return nextIndex;
}

function findGap(
  blockMap: string[],
  gapSize: number,
  cutOffIndex: number,
): number[] {
  const gapIndexes = [];
  for (let i = blockMap.length; i > 0; i--) {
    const el = blockMap[i];
    if (i <= cutOffIndex) break;
    if (el === ".") gapIndexes.push(i);
    if (gapIndexes.length === gapSize) break;
    if (el !== ".") gapIndexes.length = 0;
  }
  if (gapIndexes.length < gapSize) gapIndexes.length = 0;
  return gapIndexes;
}

function findEnd(blockMap: string[], startIndex: number, element: string) {
  let nextIndex = -1;
  for (let i = startIndex; i < blockMap.length; ++i) {
    const el = blockMap[i];
    if (element !== el) {
      nextIndex = i - 1;
      break;
    }
  }
  return nextIndex;
}

function moveFiles(
  blockMap: string[],
  elementsToSkip: string[] = [],
  startIndex: number,
): [string[], string[], number, boolean] {
  const moved = [...blockMap];

  // find start of non - dot index
  let nextNonDotStartIndex = findBlock(blockMap, startIndex, false);
  let element = blockMap[nextNonDotStartIndex];
  if (elementsToSkip.includes(element)) {
    while (true) {
      nextNonDotStartIndex = findBlock(
        blockMap,
        nextNonDotStartIndex + 1,
        false,
      );
      element = blockMap[nextNonDotStartIndex];
      if (elementsToSkip.includes(element)) continue;
      if (nextNonDotStartIndex === -1) {
        return [moved, elementsToSkip, -1, true];
      }
      break;
    }
  }

  const nextEnd = findEnd(blockMap, nextNonDotStartIndex, element);
  const length = nextEnd - nextNonDotStartIndex + 1;
  // console.log({ nextNonDotStartIndex, nextEnd, length, element });

  const gap = findGap(moved, length, nextNonDotStartIndex);
  if (gap.length) {
    const end = gap[gap.length - 1];
    moved.splice(end, length, ...new Array(length).fill(element));
    moved.splice(nextNonDotStartIndex, length, ...new Array(length).fill("."));
    return [
      moved,
      [...elementsToSkip, element],
      nextNonDotStartIndex + length,
      false,
    ];
  }

  return [
    moved,
    [...elementsToSkip, element],
    nextNonDotStartIndex + length,
    false,
  ];
}

function calculateTotal(blockMap: string[]): number {
  return blockMap.reduce(
    (acc, val, index) => val !== "." ? acc + (parseInt(val, 10) * index) : acc,
    0,
  );
}

function printBlockMap(blockMap: string[], isReversed: boolean = false) {
  console.log(isReversed ? blockMap.toReversed().join("") : blockMap.join(""));
}
console.log({ diskMap });
const blockMap = generateBlockMap(diskMap);

let workingBlockMap = [...blockMap].toReversed();
let unmoveable;
let iter = 0;
let startIndex = 0;
printBlockMap(workingBlockMap, true);
while (true) {
  const [movedBlockMap, newUnmoveable, nextStartIndex, end] = moveFiles(
    workingBlockMap,
    unmoveable,
    startIndex,
  );
  startIndex = nextStartIndex;
  workingBlockMap = movedBlockMap;
  unmoveable = newUnmoveable;
  // printBlockMap(workingBlockMap, true);
  console.log(++iter);
  if (end) break;
}

// console.log({ workingBlockMap: workingBlockMap.toReversed() });
console.log({ total: calculateTotal(workingBlockMap.toReversed()) });
