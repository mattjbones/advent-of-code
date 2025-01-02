import { Matrix } from "@izure/coord-matrix2d";

function printGrid<T>(grid: Matrix<T>) {
  console.log(
    grid.as2DArray().map((line) => line.join(" ")).join(
      "\n",
    ),
  );
}

function findAntennas(grid: Matrix<string>): Map<string, number[][]> {
  const antennaLocs = new Map<string, number[][]>();

  for (let row = 1; row < width + 1; row++) {
    for (let col = 1; col < height + 1; col++) {
      const element = grid.getElement(row, col);
      if (element === ".") continue;
      if (!antennaLocs.has(element)) {
        antennaLocs.set(element, []);
      }
      antennaLocs.get(element)?.push([row, col]);
    }
  }

  return antennaLocs;
}

function addAntinodesForElement(
  key: string,
  elementLocs: number[][],
  grid: Matrix<string>,
  antiNodeGrid: Matrix<string>,
) {
  const uniquePairs: number[][][] = [];
  elementLocs.flatMap((_, index) =>
    elementLocs.map((_, innerIndex) =>
      index !== innerIndex ? [index, innerIndex] : []
    )
  ).map((pair) =>
    pair.length > 0
      ? [
        elementLocs[pair[0]],
        elementLocs[pair[1]],
      ]
      : undefined
  ).forEach((pair) =>
    pair?.length && !uniquePairs.find((val) =>
      JSON.stringify(pair) === JSON.stringify(val.toReversed())
    ) &&
    uniquePairs.push(pair)
  );

  const valid = [".", key];
  const onlyKeyElements = grid.elements.map((element) =>
    valid.includes(element) ? element : "."
  );
  const onlyKeyGrid = Matrix.Create(width, height, onlyKeyElements);

  printGrid(onlyKeyGrid);
  console.log({ elementLocs, uniquePairs });

  // x,y, dx, dy
  // upper x + dx, y + dy
  // lower x - dx, y - dy
  uniquePairs.forEach((pair) => {
    //c = sqrt a2 + b2
    const deltaX = pair[0][0] - pair[1][0];
    const deltaY = pair[0][1] - pair[1][1];

    const [lower, upper] = pair[0][0] > pair[1][0]
      ? [pair[0], pair[1]]
      : [pair[1], pair[0]];

    let lowerX = lower[0] - deltaX;
    let lowerY = lower[1] - deltaY;
    while (antiNodeGrid.reachable(lowerX, lowerY)) {
      if (antiNodeGrid.getElement(lowerX, lowerY) === ".") {
        antiNodeGrid.setElement(lowerX, lowerY, "#");
      }
      lowerX = lowerX - deltaX;
      lowerY = lowerY - deltaY;
    }

    let upperX = upper[0] + deltaX;
    let upperY = upper[1] + deltaY;
    while (antiNodeGrid.reachable(upperX, upperY)) {
      if (antiNodeGrid.getElement(upperX, upperY) === ".") {
        antiNodeGrid.setElement(upperX, upperY, "#");
      }
      upperX = upperX + deltaX;
      upperY = upperY + deltaY;
    }
  });
  // printGrid(antiNodeGrid);
}

function countInstancesOf<T>(grid: Matrix<T>, val: T): number {
  return grid.elements.filter((element) => element === val).length;
}

// const data = Deno.readTextFileSync("./sample");
const data = Deno.readTextFileSync("./input");

const lines = data.split("\n").map((line) => line.split(""));
const width = lines[0].length;
const height = lines.length;
console.log({ width, height });

const grid = Matrix.Create(width, height, lines.flat());
printGrid(grid);

const workingGrid = grid.clone;
const antiNodeGrid = grid.clone;
const antennaLocs = findAntennas(workingGrid);
antennaLocs.forEach((val, key) => {
  console.log(key);
  addAntinodesForElement(key, val, workingGrid, antiNodeGrid);
});

const countOfMoreThan1AntennaType = antennaLocs.entries().reduce((
  acc,
  [_, locs],
) => locs.length > 1 ? acc + locs.length : acc, 0);
const antiNodeCount = countInstancesOf(antiNodeGrid, "#");

console.log({ antiNodeCount, countOfMoreThan1AntennaType });
console.log({
  total: antiNodeCount + countOfMoreThan1AntennaType,
});

console.log({ antennaLocs });
printGrid(antiNodeGrid);

//279 - 300
