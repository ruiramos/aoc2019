const fs = require('fs');

const data = fs.readFileSync('./data.txt', 'utf8').trim();
const w = 25;
const h = 6;

let min_zeros = Infinity;
let zeros = 0;
let ones = 0;
let twos = 0;
let answer;

for(let i = 0; i < data.length; i++){
  if(i && (i % (w*h) === 0)){
    if(zeros < min_zeros){
      min_zeros = zeros;
      answer = ones * twos;
    }
    ones = twos = zeros = 0;
  }

  switch(data[i]){
    case "0":
      zeros++;
      break;
    case "1":
      ones++;
      break;
    case "2":
      twos++;
      break;
  }
}

console.log(answer);

decode(data);

function decode(data){
  let layers = data.length / (w*h);
  for(let i = 0; i < w*h; i++){
    let pixel = '2';
    for(let j = 0; j < layers; j++){
      let index = i + j * (w*h);
      if(data[index] != '2'){
        pixel = data[index];
        break;
      }
    }

    if(i % w*h === 0) process.stdout.write('\n');
    process.stdout.write(pixel == '1' ? 'x' : ' ');
  }

  process.stdout.write('\n');
}

let la = [[1,0], [2,0], [3,0], [0,1], [4,1], [0,2], [4,2], [0,3], [1,3], [2,3], [3,3], [4,3], [0,4], [4,4], [0,5], [4,5]];
let le = [[0,0], [1, 0], [2, 0], [3, 0], [4,0], [0, 1], [0, 2], [0, 3], [0, 4], [0, 5], [1, 5], [2, 5], [3, 5], [4,5], [1, 2], [2, 2]];
let lf = [[0,0], [0,1], [0,2], [0,3], [0,4], [0,5], [1,0], [2,0], [3,0], [4,0], [1,2], [2,2], [3,2], [4,2]];
let li = [[0,0], [1,0], [2,0], [1,1], [1,2], [1,3], [1,4], [1,5], [0,5],[2,5]];
let ll = [[0,0], [0,1], [0,2], [0,3], [0,4], [0,5], [1,5], [2,5], [3,5], [4,5]];
let ln = [[0,0], [0,1], [0,2], [0,3], [0,4], [0,5], [1,0], [2,1], [3,2], [4,3], [5,4], [6,5], [6,4],[6,3],[6,2],[6,1], [6,0]];
let lo = [[0,1], [0,2], [0,3], [0,4], [1,0], [2,0], [3,0], [4,0], [1,5], [2,5], [3,5], [4,5], [5,4], [5,3], [5,2], [5,1]];
let lp = [[0,0], [0,1], [0,2], [0,3], [0,4], [0,5], [1,0], [2,0], [3,0], [4,0], [4,1], [4,2], [3,2], [2,2], [1,2]];
let ls = [[0,0],[1,0], [0, 1], [0, 2], [3, 0], [2, 0], [1, 2], [2, 2], [3,2], [3, 3], [3, 4], [3, 5], [2, 5], [1, 5], [0,5]];

function space(coords, spacing){
	return coords.map(([x,y]) => [x+spacing, y]);
}

function assemble(phrase){
  let pos = 0;
  let result = phrase.flatMap((letter, i) => {
    if(i > 0){
      pos += max_letter_width(phrase[i-1]) + 2;
    }
    return space(letter, pos);
  });
  return result;
}

function max_letter_width(letter){
  return Math.max(...letter.map(([x]) => x)) || 0;
}

let e = encode(assemble([lo, ll, la]), 100, w, h);

fs.writeFileSync('./image.jpg.txt', e.join(''));

decode(e);

function encode(white_pixels, layers, w, h){
  let answer = [];
  for(let y = 0; y < h; y++){
    for(let x = 0; x < w; x++){
      let point = [x,y];
      let is_white = is_white_pixel(point, white_pixels);
      let is_right_color = false;

      for(let i = layers - 1; i >= 0; i--){
        let index = x + (y*w) + (i * w * h);

        if(!is_right_color && Math.random() < .65){
          answer[index] = is_white ? 1 : 0;
        } else if (is_right_color && Math.random() < .85){
          answer[index] = 2;
        } else {
          answer[index] = !is_white ? 1 : 0;
        }

        let pixel = answer[index];

        if(is_white && pixel == 1 || !is_white && pixel == 0){
          is_right_color = true;
        } else if(pixel != 2){
          is_right_color = false;
        }

        if(!is_right_color && i == 0){
          answer[index] = is_white ? 1 : 0;
        }

      }

    }
  }
  return answer;
}

function is_white_pixel(p, list){
  let ans = list.filter(([x, y]) => 
    x == p[0] && y == p[1]
  );
  return !!ans.length;
}
