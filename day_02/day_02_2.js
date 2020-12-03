console.log(document.documentElement.innerText
	.split("\n")
	.filter(line => line != "")
	.map(line => {
		const [, pos1, pos2, letter, word] = line.match(/^([0-9]+)-([0-9]+)\s(\D):\s(\D*)/) || []
		return {pos: [pos1, pos2], letter, word}
	})
	.map(data => {return {pos: data.pos.map(pos => Number(pos)-1), ...data}})
	.filter(data => data.pos.every(pos => data.word.charAt(pos) == data.letter))
	.length
)
