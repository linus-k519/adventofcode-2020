class Mask {
  constructor(mask) {
    // Extract positive mask with all forced ones
    this.pos = BigInt('0b' + mask.replaceAll('X', '0'))
    // ... and negative mask with all forced zeros
    this.neg = BigInt('0b' + mask.replaceAll('X', '1'))
  }

  apply(val) {
    // Apply positve mask (forced ones)
    // and negative mask (forced zeros) to value
    val |= this.pos
    val &= this.neg
    return val
  }
}

mem = {}
mask = null

function update_mask(maskMatch) {
  mask = new Mask(maskMatch[1])
}

function update_mem(memMatch) {
  // Parse memory positon and value
  pos = parseInt(memMatch[1])
  val = BigInt(memMatch[2])
  mem[pos] = mask.apply(val)
}

// Parse document
document.documentElement.innerText.split('\n')
.forEach(line => {
  maskMatch = line.match(/mask = ([X01]+)/)
  memMatch = line.match(/mem\[(\d+)\] = (\d+)/)
  if (maskMatch) update_mask(maskMatch)
  else if (memMatch) update_mem(memMatch)
});

// Calculating sum
sum = Object.values(mem).reduce((acc, cur) => acc + cur)
console.log('Sum', sum.toString())
