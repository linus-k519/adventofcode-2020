console.log((() => {
		karte = document.documentElement.innerText
			.split("\n")
			.filter(line => line != "")
			.map(line => line.split("").map(letter => letter == "#" ? true : false)
		)

		fahrten = [[1,1], [3,1], [5,1], [7,1], [1,2]]

		return fahrten.reduce((acc_product, fahrt) =>
			acc_product * karte.map((line, lineNr) => line[lineNr * fahrt[0] % line.length])
			.filter((_, lineNr) => lineNr % fahrt[1] == 0)
			.reduce((acc_trees, cur) => acc_trees + cur, 0)
		)
	})()
)
