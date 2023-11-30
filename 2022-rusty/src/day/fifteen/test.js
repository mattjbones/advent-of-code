

const target = [7, 8];
const coords = [];

let count = 1;

while (count < 2) {

    const lower_y = target[1] - count;
    const upper_y = target[1] + count;

    let x = lower_y - target[1];
    for (let y = lower_y; y<upper_y+1; y++){

        if (y < target[1]) {
            x += count  
        }else if (y == target[1]){
            x = target[0];
        }else{
            x -= count 
        }

        console.log({y,x});
        const lower_x = x - count;
        const upper_x = x + count;

        // console.log({lower_x, upper_x});
        for (let x = lower_x; x<upper_x+1; x++){
            coords.push([x,y]);
        }
    }

    ++count;
}


printCoords(coords, target);

function printCoords(coords, target) {
  let max_x = 0;
  let max_y = 0;
  let min_y = Infinity;
  let min_x = Infinity;

  [...coords, target].forEach(([x, y]) => {
    max_x = x > max_x ? x : max_x;
    min_x = x < min_x ? x : min_x;
    max_y = y > max_y ? y : max_y;
    min_y = y < min_y ? y : min_y;
  });

//   console.log({ max_y, min_y, max_x, min_x });

  for (let y = min_y - 1; y < max_y + 2; y++) {
    for (let x = min_x - 2; x < max_x + 2; x++) {
      let coord = coords.find(
        (c) => JSON.stringify(c) === JSON.stringify([x, y])
      );
      if (x == target[0] && y == target[1]) {
        process.stdout.write("S");
      }else if (coord) {
        process.stdout.write("#");
      } else {
        process.stdout.write(".");
      }
    }
    process.stdout.write('\n');
    }
}

