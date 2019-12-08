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


let layers = data.length / (w*h);
for(let i = 0; i < w*h; i++){
  let pixel = '2';
  for(let j = 0; j < layers; j++){
    let index = i + j * (w*h);
    if(data[index] !== '2'){
      pixel = data[index];
      break;
    }
  }

  if(i % w*h === 0) process.stdout.write('\n');
  process.stdout.write(pixel == '1' ? 'x' : ' ');
}

