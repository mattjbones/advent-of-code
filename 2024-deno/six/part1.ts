import { Matrix } from "@izure/coord-matrix2d";


const map = Deno.readTextFileSync("./input");

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
const visitedGrid = Matrix.Create(width, height, new Array(width * height).fill('.'));
console.log(grid);

let [currRow, currCol] = [startRow, startCol];
let direction = startDirection;

console.log({currRow, currCol});
while(true) {
  if(currRow >= width || currCol >= height || currCol < 1 || currRow < 1){
    break;
  }
  
  const [nextRow, nextCol] = move(direction, [currRow, currCol]);
  if(nextRow > width || nextCol > height || nextCol < 1 || nextRow < 1){
    break;
  }
  const nextElement = grid.getElement(nextRow, nextCol);

  console.log({nextRow, nextCol})
  if(nextElement === "#"){
    direction = rotate(direction);
  }else{
    visitedGrid.setElement(currRow, currCol, "#");
    [currRow, currCol] = [nextRow, nextCol] 
  }
  
}

console.log(visitedGrid.as2DArray().map(row => row.join("")).join("\n"))
console.log(visitedGrid.elements.filter((val) => val === "#").length + 1);

