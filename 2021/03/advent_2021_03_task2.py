import copy
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
input_data_np = np.array(input_data)
input_data_transposed = input_data_np.transpose()

most_common_bits = []
least_common_bits = []


# Because the ndarray is now transposed, the numbers for gamma and epsilon rate are simply the rows
for row in input_data_transposed:
    # Convert numpy array to list...
    row = row.tolist()
    # ... to be able to find the most and least common elements
    count0 = row.count(0)
    count1 = row.count(1)
    print(f'{count0=} {count1=}')

    # If 0 and 1 are equally common, keep values with a 1 in the position being considered.
    most_common = 1 if count1 >= count0 else 0
    # If 0 and 1 are equally common, keep values with a 0 in the position being considered.
    least_common = 0 if count0 >= count1 else 1

    # 'Add' the current bit
    most_common_bits.append(most_common)
    least_common_bits.append(least_common)

print('most common bits', most_common_bits)


def calculate_rating(rating: List[List[int]], common_bits: List[int]) -> int:
    for bit_position in range(len(input_data[0])):
        if len(rating) == 1:
            # If you only have one number left, stop
            break
        # Keep only numbers selected by the bit criteria for the type of rating value for which you are searching.
        # Discard numbers which do not match the bit criteria.
        common = common_bits[bit_position]
        rating = list(filter(lambda number: number[bit_position] == common, rating))
        print(f'{rating=}')

    rating = rating[0]
    rating = [str(x) for x in rating]
    rating = int(''.join(rating), base=2)
    return rating


print('oxygen')
oxygen_generator_rating = calculate_rating(copy.deepcopy(input_data), most_common_bits)
print('co2')
co2_scrubber_rating = calculate_rating(copy.deepcopy(input_data), least_common_bits)

print(f'{oxygen_generator_rating=} {co2_scrubber_rating=}')

result = oxygen_generator_rating * co2_scrubber_rating
print(result)
