import { Matrix } from "@izure/coord-matrix2d";

const xmasses = Deno.readTextFileSync("./input");

const lines = xmasses.split("\n").map((line) => line.split(""));
const width = lines[0].length;
const height = lines.length;

const forwards = "XMAS".split("").join("");
const backwards = [...forwards].toReversed().join("");

const grid = Matrix.Create(width, height, lines.flat());
console.log(lines.map((line) => line.join(" ")).join("\n"));

function processElement(row: number, col: number, queue: string[]) {
  const element = grid.getElement(row, col);
  queue.push(element);
}

let total = 0;
function processQueue(queue: string[]) {
  let workingTotal = 0;
  let working = queue.join("");
  while (true) {
    const indexOfForwards = working.indexOf(forwards);
    const indexOfBackwards = working.indexOf(backwards);
    if (indexOfBackwards === -1 && indexOfForwards === -1) {
      break;
    }

    if (indexOfBackwards !== -1 && indexOfForwards === -1) {
      working = working.slice(indexOfBackwards + forwards.length);
      ++workingTotal;
      continue;
    }

    if (indexOfForwards !== -1 && indexOfBackwards === -1) {
      working = working.slice(indexOfForwards + forwards.length);
      ++workingTotal;
      continue;
    }

    if (indexOfForwards < indexOfBackwards) {
      working = working.slice(indexOfForwards + 1);
      ++workingTotal;
    } else {
      working = working.slice(indexOfBackwards + 1);
      ++workingTotal;
    }
  }

  console.log("Total:", workingTotal, " Queue:", queue.join(""));
  // if(workingTotal > 0){
  // }

  total += workingTotal;
}

/*
[ x, x, x, x, x]
[ x, x, x, x, x]
[ x, x, x, x, x]
[ x, x, x, x, x]
[ x, x, x, x, x]
*/

const queue: string[] = [];
//horizontal
for (let row = 1; row < width + 1; row++) {
  for (let col = 1; col < height + 1; col++) {
    processElement(row, col, queue);
  }
  processQueue(queue);
  queue.length = 0;
}

console.log();

//vertical
for (let col = 1; col < height + 1; col++) {
  for (let row = 1; row < width + 1; row++) {
    processElement(row, col, queue);
  }
  processQueue(queue);
  queue.length = 0;
}

console.log();

//diagonal
const queues: string[][] = [];
for (let i = 1; i < width + 1; i++) {
  for (let j = 1; j < height + 1; j++) {
    if (!queues[i + j]) {
      queues[i + j] = [];
    }
    queues[i + j].push(grid.getElement(i, j));
  }
}
for (const queue of queues) {
  if (queue) processQueue(queue);
}
queues.length = 0;
queue.length = 0;

console.log();

console.log(lines.map((line) => line.join(" ")).join("\n"));

console.log();

const length = width;
const diagonalLines = (length + length) - 1;
const midPoint = (diagonalLines / 2) + 1;

let itemsInDiagonal = 0;
for (let i = 1; i <= diagonalLines; i++) {
  let rowIndex;
  let columnIndex;

  if (i <= midPoint) {
    itemsInDiagonal++;
    for (let j = 0; j < itemsInDiagonal; j++) {
      rowIndex = i - j;
      columnIndex = width - j;
      processElement(rowIndex, columnIndex, queue);
    }
    processQueue(queue);
    queue.length = 0;
  } else {
    itemsInDiagonal--;
    for (let j = 0; j < itemsInDiagonal; j++) {
      rowIndex = length - j;
      columnIndex = width - j - (i - length);
      processElement(rowIndex, columnIndex, queue);
    }
    processQueue(queue);
    queue.length = 0;
  }
}

console.log();

/*
  0,0 0,1 0,2  0,width
  1,0 1,1 1,2
  2,0 2,1 2,2

  height,0     height,width

  processElement(0,0);
  queue.length = 0

  processElement(1,0);
  processElement(0,1);
  queue.length = 0

  processElement(0,2);
  processElement(1,1);
  processElement(2,0);

  ---

  processElement(2,0);

  processElement(1,0);
  processElement(2,1);

  processElement(0,0);
  processElement(2,1);


*/

console.log({ total });
