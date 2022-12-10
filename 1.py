#!/usr/bin/env python3

calories_by_elf=list()

with open('1.input') as paper:
    count = 0

    while count_or_blank := paper.readline():
        count_or_blank = count_or_blank.strip()

        # each blank line delimits where the data for an elf ends, so we will
        # save the current elf when we reach that line
        if len(count_or_blank) == 0:
            calories_by_elf.append(count)
            count = 0
            continue

        count += int(count_or_blank)
    
    # the last elf may not have a blank line
    if count != 0:
        calories_by_elf.append(count)

# reverse sort for ranking
calories_by_elf.sort(reverse=True)

# part 1
print(f'top_1={calories_by_elf[0]}')

# part 2
top_3 = calories_by_elf[:3]
print(f'{top_3=}')
print(f'{sum(top_3)}')
