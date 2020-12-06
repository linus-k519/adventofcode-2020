// Not working correctly

document.documentElement.innerText
.split("\n\n")
.map(g => g.split("\n"))
.map(g => g.map(p => p.split("")
    .reduce((acc, cur) => acc.add(cur), new Set())))
.map(g => g.reduce((acc, cur) => 
    acc === new Map() ? cur : new Set([...acc].filter(x => cur.has(x)))))
.reduce((acc, cur) => acc + cur.size, 0)
