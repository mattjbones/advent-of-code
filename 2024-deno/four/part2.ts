import { Matrix } from "@izure/coord-matrix2d";

const xmasses = Deno.readTextFileSync("./input");

const lines = xmasses.split("\n").map((line) => line.split(""));
const width = lines[0].length;
const height = lines.length;

const forwards = "MAS"
const backwards = [..."MAS"].toReversed().join("")

const grid = Matrix.Create(width, height, lines.flat());
console.log(lines.map((line) => line.join(" ")).join("\n"), "\n");


// grab 3x3 local grids until the end 
// 1,1  1,2  1,3
// 2,1  2,2  2,3
// 3,1  3,2  3,3

let total = 0;
let currRow = 2;
let currCol = 2
while (currRow < width  && currCol < height ) {
  const local = Matrix.GetLocalMatrix(grid, currRow, currCol);
  // console.log({currRow, currCol})
  if(local.getElement(2,2) === "A"){
    console.log("Found: ", local.elements)
    const diagTopBottom = `${local.getElement(1,1)}${local.getElement(2,2)}${local.getElement(3,3)}`
    const diagLeftRight = `${local.getElement(1,3)}${local.getElement(2,2)}${local.getElement(3,1)}`
    const topBottomMatch = diagTopBottom === forwards || diagTopBottom === backwards;
    const leftRightMatch = diagLeftRight === forwards || diagLeftRight === backwards;
    // console.log({diagLeftRight, diagTopBottom, pass: topBottomMatch && leftRightMatch})
    if(topBottomMatch && leftRightMatch) total+=1;
  }

  if(currRow < height - 1){
    currRow++;
  }else if(currCol < width - 1){
    currRow = 2
    currCol++;
  }else{
    break;
  }
}

console.log({total})