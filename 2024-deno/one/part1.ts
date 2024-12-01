import { BinaryHeap, descend  } from "@std/data-structures";


const left = new BinaryHeap<number>(descend)
const right = new BinaryHeap<number>(descend)
const data = await Deno.readTextFile("./input");
for(const line of data.split("\n")){
  const [leftString, rightString] = line.split(/[ ,]+/)
  left.push(parseInt(leftString, 10))
  right.push(parseInt(rightString, 10))
}

if (left.length !== right.length){
  console.error("Lists aren't even")
  Deno.exit(1)
}


const diff = []
while(left.length){
  //@ts-expect-error above check ensures it's not undefined
  diff.push(Math.abs(left.pop() - right.pop()));
}

console.log(diff.reduce((acc, curr) => acc + curr, 0));





