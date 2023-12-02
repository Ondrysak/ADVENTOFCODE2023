#Your calculation isn't quite right.
#It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

import re

# Find first number in a string
def find_first_number(input_string):
    match = re.search(r'(one|1|two|2|three|3|four|4|five|5|six|6|seven|7|eight|8|nine|9)', input_string)
    return match.group()

# Find last number in a string
def find_last_number(input_string):
    matches = re.findall(r'(one|1|two|2|three|3|four|4|five|5|six|6|seven|7|eight|8|nine|9)', input_string)
    return matches[-1]

# Convert text number into digit
def text_to_digit(num):
    if num == "one":
        num = "1"
    if num == "two":
        num = "2"
    if num == "three":
        num = "3"
    if num == "four":
        num = "4"
    if num == "five":
        num = "5"
    if num == "six":
        num = "6"
    if num == "seven":
        num = "7"
    if num == "eight":
        num = "8"
    if num == "nine":
        num = "9"
    return num

result = 0

# Open the file in read mode
with open('input_real.txt', 'r') as file:
    # Iterate over each line in the file
    for line in file:
        first = text_to_digit(find_first_number(line))
        last = text_to_digit(find_last_number(line))
        result = result + int(first + last)

# Print result
print(result)