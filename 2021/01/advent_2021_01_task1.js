document.documentElement.innerText
    .split('\n')
    // Convert lines to number
    .map(line => Number.parseInt(line))
    // Remove lines with invalid numbers, i.e. the last empty line
    .filter(number => !Number.isNaN(number))
    .reduce((accumlator, current) => {
        // Increment ascending, if ascending
        accumlator.ascending += accumlator.previous < current;
        accumlator.previous = current;
        return accumlator
    }, // Start value: 0 ascending numbers
        // and previous is max so that the first number does not get counted as ascending
        {ascending: 0, previous: Number.MAX_SAFE_INTEGER})
    .ascending