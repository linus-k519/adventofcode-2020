document.documentElement.innerText
.split("\n\n")
.flatMap(g => g.replaceAll("\n", ""))
.map(g => g.split("")
    .reduce((acc, cur) => acc.add(cur), new Set()))
.map(g => g.size)
.reduce((acc, cur) => acc + cur)
