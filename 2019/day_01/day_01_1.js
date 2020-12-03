document.documentElement.innerText
  .split("\n")
  .filter(line => line != "")
  .map(n => parseInt(n / 3) - 2)
  .reduce((acc, cur) => acc + cur)
