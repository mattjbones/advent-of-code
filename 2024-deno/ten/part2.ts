import { Matrix } from "@izure/coord-matrix2d";

// const map = Deno.readTextFileSync("./example4"); //3
// const map = Deno.readTextFileSync("./sample"); //81
const map = Deno.readTextFileSync("./input");

const lines = map.split("\n");
const width = lines[0].length;
const height = lines.length;

const grid = Matrix.Create(
  width,
  height,
  lines.flatMap((line) => line.split("").map((val) => parseInt(val, 10))),
);

function printGrid<T extends number | string>(grid: Matrix<T>) {
  console.log(
    grid.as2DArray().map((row) =>
      row.map((val) => typeof val === "string" ? val : isNaN(val) ? "." : val)
        .join(" ")
    ).join("\n"),
  );
}

enum Direction {
  Left,
  Right,
  Up,
  Down,
}
function getElementPositionForDirection(
  grid: Matrix<number>,
  position: [number, number],
  direction: Direction,
): [[number, number], number] {
  switch (direction) {
    case Direction.Left: {
      const nextLeft: [number, number] = [position[0], position[1] + 1];
      if (nextLeft[1] > grid.row) {
        return [position, grid.getElement(...position)];
      }
      const nextLeftElement = grid.getElement(...nextLeft);
      return [nextLeft, nextLeftElement];
    }
    case Direction.Right: {
      const nextRight: [number, number] = [position[0], position[1] - 1];
      if (nextRight[1] < 1) return [position, grid.getElement(...position)];
      const nextRightElement = grid.getElement(...nextRight);
      return [nextRight, nextRightElement];
    }
    case Direction.Up: {
      const nextUp: [number, number] = [position[0] + 1, position[1]];
      if (nextUp[0] > grid.col) {
        return [position, grid.getElement(...position)];
      }
      const nextUpElement = grid.getElement(...nextUp);
      return [nextUp, nextUpElement];
    }
    case Direction.Down: {
      const nextUp: [number, number] = [position[0] - 1, position[1]];
      if (nextUp[0] < 1) {
        return [position, grid.getElement(...position)];
      }
      const nextUpElement = grid.getElement(...nextUp);
      return [nextUp, nextUpElement];
    }
  }
}

printGrid(grid);

const directions = [
  Direction.Up,
  Direction.Left,
  Direction.Right,
  Direction.Down,
];
// find starting positions
const starts: [number, number][] = [];
for (let r = 1; r < width + 1; r++) {
  for (let c = 1; c < height + 1; c++) {
    const element = grid.getElement(r, c);
    if (element === 0) starts.push([r, c]);
  }
}

// DFS
let endCount = 0;
while (starts.length) {
  const visited: string[] = [];
  const queue: [number, number][] = [];
  queue.push(starts.pop()!);

  while (queue.length) {
    const position = queue.pop();
    if (!position) break;
    const element = grid.getElement(...position);
    if (element === 9) endCount++;
    else {
      visited.push(position.join(","));
    }
    const options = directions.map((direction) => {
      const [nextPosition, nextElement] = getElementPositionForDirection(
        grid,
        position,
        direction,
      );
      if (
        !isNaN(nextElement) && nextElement - element === 1 &&
        !visited.includes(nextPosition.join(","))
      ) {
        return nextPosition;
      }
    }).filter(Boolean);

    if (options.length) {
      queue.push(options.pop()!);
    }
    if (options.length) {
      //@ts-expect-error length check above?
      starts.push(...options);
    }
  }
}

console.log({ endCount });
