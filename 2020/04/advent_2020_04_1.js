document.documentElement.innerText
  .split("\n\n")
  .map(p => p.replaceAll("\n", " "))
  .map(p => p.split(" "))
  .map(p => Object.fromEntries(p.map(kv => kv.split(":"))))
  .filter(p => p.byr && p.iyr && p.eyr && p.hgt && p.hcl && p.ecl && p.pid)
  .length
