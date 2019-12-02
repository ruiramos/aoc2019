const assert = require('assert');
const fs = require('fs');
const fuel = (m) => Math.floor(parseInt(m, 10) / 3) - 2;

const file = fs.readFileSync('./fuel.txt', 'utf-8');
const massArray = file.split('\n');

const testInputs = [
  [12, 2],
  [14, 2],
  [1969, 654],
  [100756, 33583]
];

testInputs.forEach(([input, expected]) => {
  assert.equal(fuel(input), expected);
});

const fuels = massArray.map(m => fuel(m)).filter(f => !!f);
const sum = (a) => a.reduce((acc, el) => acc + el, 0);

console.log(JSON.stringify(fuels), sum(fuels));

const fuels2 = massArray.map(m => {
  let result = 0;
  let mass = m;
  while(fuel(mass) > 0){
    let calc = fuel(mass);
    mass = calc;
    result += calc;
  }

  return result;
})

console.log(JSON.stringify(fuels2), sum(fuels2));
