fuel = weight => {
    if (weight <= 0) {
        return 0;
    }
    else {
    	new_weight = parseInt(weight / 3) - 2
    	return new_weight + fuel(new_weight)
    }
}

document.documentElement.innerText
    .split("\n")
    .filter(line => line != "")
    .map(n => parseInt(n))
    .map(fuel)
    .reduce((acc, cur) => acc + cur, 0)
