document.documentElement.innerText
  .split("\n\n")
  .filter(line => line != "")
  .map(p => p.replaceAll("\n", " "))
  .map(p => p.split(" "))
  .map(p => Object.fromEntries((p.map(kvs => kvs.split(":")))))
  .filter(p => p.byr && p.iyr && p.eyr && p.hgt && p.hcl && p.ecl && p.pid && p.cid)
  .length
