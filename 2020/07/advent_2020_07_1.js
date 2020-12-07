// Not working correctly

document.documentElement.innerText.split("\n")
.map(l => l.match(/\w+ \w+ bags contain.*\d shiny gold bags?/))
.filter(l => l != null)
.length
