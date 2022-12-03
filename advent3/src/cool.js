const fs = require("fs");

const data = fs.readFileSync("./src/input.txt").toString().split("\n");
let iterator = data[Symbol.iterator]();
let total = 0;
while (true) {
  const { value: elf1 } = iterator.next();
  const { value: elf2 } = iterator.next();
  const { value: elf3 } = iterator.next();
  if (!elf1 || !elf2 || !elf3) break;
  for (let character of elf1.split("")) {
    if (elf2.indexOf(character) > -1 && elf3.indexOf(character) > -1) {
      let charCode = character.charCodeAt();
      total +=
        character == character.toLowerCase()
          ? charCode - 96
          : charCode - 64 + 26;
      break;
    }
  }
}

console.log("i finish", total);
