
const left = []
const right = new Map<number, number>() ;
const data = await Deno.readTextFile("./input");
for(const line of data.split("\n")){
  const [leftString, rightString] = line.split(/[ ,]+/)
  left.push(parseInt(leftString, 10))

  const rightNumber = parseInt(rightString, 10);
  const initialNumber = right.get(rightNumber) ?? 0
  right.set(rightNumber, initialNumber + 1);
}

const diff = []
while(left.length){
  const currNumber = left.pop();
  if(typeof currNumber === "undefined"){
    console.error('ded')
    Deno.exit(1)
  }

  const countOfNumber = right.get(currNumber) ?? 0 
  diff.push(currNumber * countOfNumber);
}

console.log(diff.reduce((acc, curr) => acc + curr, 0));





