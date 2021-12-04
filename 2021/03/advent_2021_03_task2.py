from typing import List

import numpy as np

# Read input from file
with open('input.txt') as input_file:
    # Split line so that we get an array of lines
    input_data = input_file.read().split('\n')

# Remove empty lines, i.e. the last one
input_data = filter(lambda row: bool(row), input_data)
# Convert each line into a list of characters
input_data = [list(line) for line in input_data]
# Convert each character ('0' or '1') to an int (0 or 1)
input_data = [[int(n) for n in row] for row in input_data]
# Create numpy array and transpose it, so that rows are no columns and vice versa
input_data = np.array(input_data)
input_data = input_data.transpose()

gamma_rate = 0
epsilon_rate = 0

# Because the ndarray is now transposed, the numbers for gamma and epsilon rate are simply the rows
for row in input_data:
    # Convert numpy array to list...
    row = row.tolist()
    # ... to be able to find the most and least common elements
    most_common = max(set(row), key=row.count)
    least_common = min(set(row), key=row.count)

    # 'Add' the current bit as the least significant bit
    gamma_rate <<= 1
    gamma_rate |= most_common

    epsilon_rate <<= 1
    epsilon_rate |= least_common

# print(f'{gamma_rate=} {epsilon_rate=}')

result = gamma_rate * epsilon_rate
print(result)
