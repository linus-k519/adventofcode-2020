console.log(document.documentElement.innerText
	.split("\n")
	.filter(line => line != "")
	.map(line => {
		const [, min, max, letter, word] = line.match(/^([0-9]+)-([0-9]+)\s(\D):\s(\D*)/) || []
		return {min, max, letter, word}
	})
	.reduce((acc, cur) => {
		amount = (cur.word.match(new RegExp(cur.letter, "g")) || []).length
		return acc + (cur.min <= amount && amount <= cur.max ? true : false)
	}, 0)
)
