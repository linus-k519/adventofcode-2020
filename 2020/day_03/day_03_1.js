console.log(document.documentElement.innerText
	.split("\n")
	.filter(line => line != "")
	.map(line => line.split("").map(letter => letter == "#" ? true : false))
	.reduce((acc, cur, lineNr) => acc + (cur[(lineNr * 3) % cur.length]), 0)
)
