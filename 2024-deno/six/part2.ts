import { Matrix } from "@izure/coord-matrix2d";


const map = Deno.readTextFileSync("./sample");
// const map = Deno.readTextFileSync("./input");

enum Direction {
  UP, 
  DOWN, 
  LEFT, 
  RIGHT
}

const rotate = (currDir: Direction) => {
  switch(currDir){
    case Direction.UP:
      return Direction.RIGHT;
    case Direction.DOWN:
      return Direction.LEFT;
    case Direction.LEFT:
      return Direction.UP;
    case Direction.RIGHT:
      return Direction.DOWN;
  }
};

const move = (currDir: Direction, coord: [number, number]) => {
  switch (currDir){
    case Direction.UP:
      return [coord[0] - 1, coord[1]];
    case Direction.DOWN:
      return [coord[0] + 1, coord[1]];
    case Direction.LEFT:
      return [coord[0], coord[1] - 1];
    case Direction.RIGHT:
      return [coord[0], coord[1] + 1];
  }
};

const flipDirection = (direction: Direction) => {
  switch(direction){
    case Direction.UP:
      return Direction.DOWN;
    case Direction.DOWN:
      return Direction.UP;
    case Direction.LEFT:
      return Direction.RIGHT;
    case Direction.RIGHT:
      return Direction.LEFT;
  }

}

const checkForCycle = (visitedGrid: Matrix<number>, [row, col]: [number, number], direction : Direction) => {
  // console.log(visitedGrid.getElement(row, col), direction)
  return visitedGrid.getElement(row, col) === direction;
}

const lines = map.split("\n").map((row) => row.split(""));
const width = lines[0].length;
const height = lines.length;
const [startRow, startCol]= lines.map((line, row) => line.indexOf("^") !== -1 ? [row + 1, line.indexOf("^") + 1] : undefined).filter(Boolean).flat();
const startDirection = Direction.UP;

if(!startCol || !startRow){
  console.error("Cannot find start") 
  Deno.exit(1);
}

console.log({startRow, startCol});
const grid = Matrix.Create(width, height, lines.flat());
console.log(grid);

//get path
const obstructionsToAdd = [];
const visitedGrid = Matrix.Create(width, height, new Array(width * height).fill(undefined));
let [currRow, currCol] = [startRow, startCol];
let direction = startDirection;
while(true) {
  if(currRow >= width || currCol >= height || currCol < 1 || currRow < 1){
    break;
  }
  
  const [nextRow, nextCol] = move(direction, [currRow, currCol]);
  if(nextRow > width || nextCol > height || nextCol < 1 || nextRow < 1){
    break;
  }

  const nextElement = grid.getElement(nextRow, nextCol);
  if(nextElement === "#"){
    direction = rotate(direction);
  }else{
    obstructionsToAdd.push([nextRow, nextCol, direction]);
    if(!visitedGrid.getElement(currRow, currCol)){
      visitedGrid.setElement(currRow, currCol, direction);
    }
    [currRow, currCol] = [nextRow, nextCol] 
  }
}

console.log(visitedGrid.as2DArray().map(row => row.join("")).join("\n"))
console.log({obstructionsToAdd});

// add obstructions to each coord on path and count loops
let loops = 0;
while(obstructionsToAdd.length){

  console.log(obstructionsToAdd.length);

  const workingGrid = grid.clone;
  const visitedGrid = Matrix.Create(width, height, new Array(width*height).fill(undefined));

  const currentObstruction = obstructionsToAdd.pop();
  if(!currentObstruction) break;

  const [obstructionRow, obstructionCol, directionAtPoint ] = currentObstruction;
  workingGrid.setElement(obstructionRow, obstructionCol, "#");

  let [currRow, currCol] = move(flipDirection(directionAtPoint), [obstructionRow, obstructionCol]);
  let direction = directionAtPoint;

  while(true) {
    if(currRow >= width || currCol >= height || currCol < 1 || currRow < 1){
      console.log("Reached end")
      break;
    }

    const isInLoop = checkForCycle(visitedGrid, [currRow, currCol], direction);
    if(isInLoop){
      console.log(visitedGrid.as2DArray().map(arr => arr.map(val => val ? "#" : ".").join(" ")).join("\n"))
      ++loops
      break;
    }
    
    const [nextRow, nextCol] = move(direction, [currRow, currCol]);
    if(nextRow > width || nextCol > height || nextCol < 1 || nextRow < 1){
      break;
    }
    
    const nextElement = workingGrid.getElement(nextRow, nextCol);
    if(nextElement === "#"){
      direction = rotate(direction);
    }else{
      visitedGrid.setElement(currRow, currCol, direction);
      [currRow, currCol] = [nextRow, nextCol] 
    }
  }
}


console.log("Total:", loops);  //1703
