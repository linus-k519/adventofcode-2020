const data = document.documentElement.innerText
    .split('\n')
    .map(line => Number.parseInt(line))
    .filter(number => !Number.isNaN(number))


let counter = 0
for (let i = 0; i < data.length; i++) {
    const prev = data[i] + data[i+1] + data[i+2]
    const next = data[i+1] + data[i+2] + data[i+3]
    counter += prev < next // If one is NaN, comparison is false, i.e. 0
}

console.log(counter)