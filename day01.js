console.log(document.documentElement.innerText
  .split("\n")
  .map(n => parseInt(n, 10))
  .filter(n => !isNaN(n))
  .map(n => 2020 - n)
  .filter(n => list.includes(n))
  .reduce((acc, cur) => acc * cur, 1)
)
