import { Matrix } from "@izure/coord-matrix2d";

// const data = Deno.readTextFileSync("./example1"); // 140
// const data = Deno.readTextFileSync("./example2"); // 772
// const data = Deno.readTextFileSync("./sample"); // 1930
const data = Deno.readTextFileSync("./input"); // ?

function generateGrid(data: string) {
  const parts = data.split("\n").map((part) => part.split(""));
  const grid = Matrix.Create(parts[0].length, parts.length, parts.flat());
  return grid;
}

function printGrid(grid: Matrix<string>) {
  console.log(grid.as2DArray().map((val) => val.join("")).join("\n"));
}

function findNeighbours(
  grid: Matrix<string>,
  key: string,
  [row, col]: [number, number],
): [number, number][] {
  const cardinals: [number, number][] = [[row + 1, col], [row - 1, col], [
    row,
    col + 1,
  ], [
    row,
    col - 1,
  ]];

  const neighbours = cardinals.filter((loc) => {
    if (grid.reachable(...loc)) {
      return grid.getElement(...loc) === key;
    }
    return false;
  });

  return neighbours;
}

function generateRegionData(grid: Matrix<string>) {
  const regionData = new Map<string, [number, number][][]>();
  const visited: string[] = [];
  const queue: [number, number][] = [];
  let sameGroup = false;
  let lastStart: [number, number] = [1, 1];
  queue.push([...lastStart]);
  while (queue.length) {
    const loc = queue.pop()!;
    visited.push(loc.join(","));
    const element = grid.getElement(...loc);
    if (!regionData.has(element)) regionData.set(element, []);
    if (!sameGroup) {
      regionData.get(element)!.push([]);
      sameGroup = true;
    }

    const groups = regionData.get(element)!;
    const grouping = groups[groups.length - 1];
    if (!grouping.find((coord) => coord.join(",") === loc.join(","))) {
      grouping.push(loc);
    }

    const neighbours = findNeighbours(grid, element, loc);
    neighbours.forEach((neighbourLoc) => {
      if (
        !visited.includes(neighbourLoc.join(","))
      ) {
        queue.push(neighbourLoc);
      }
    });

    if (!queue.length) {
      sameGroup = false;
      searchLoop:
      for (let row = lastStart[0]; row < grid.row + 1; row++) {
        for (let col = 1; col < grid.col + 1; col++) {
          if (!visited.includes([row, col].join(","))) {
            lastStart = [row, col];
            queue.push([row, col]);
            break searchLoop;
          }
        }
      }
    }
  }

  return regionData;
}

function calculatePerimeter(
  locations: [number, number][],
  grid: Matrix<string>,
  val: string,
): number {
  return locations.reduce((acc, loc) => {
    const [x, y] = [loc[0], loc[1]];
    const neighbours = findNeighbours(grid, val, [x, y]).length;
    const perimeter = 4 - neighbours;
    return acc + perimeter;
  }, 0);
}

function calculateCost(
  locations: [number, number][],
  grid: Matrix<string>,
  key: string,
): number {
  const area = locations.length;
  const perimeter = calculatePerimeter(locations, grid, key);
  console.log({ key, area, perimeter, cost: area * perimeter });
  return area * perimeter;
}

const grid = generateGrid(data);
printGrid(grid);

const regionData = generateRegionData(grid);
let total = 0;
regionData.forEach((locs, key) => {
  locs.forEach((coords) => {
    console.log("Processing", key, coords);
    total += calculateCost(coords, grid, key);
  });
});
console.log({ total });
