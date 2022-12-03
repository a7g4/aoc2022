import sys
import functools

lines = []

for line in sys.stdin:
    lines.append(line)

elfs = []
elf = []

for line_n in range(len(lines)):
    if lines[line_n] == "\n":
        elfs.append(elf)
        elf = []
        continue

    elf.append(int(lines[line_n]))

elf_cals = []
for elf in elfs:
    elf_cals.append(functools.reduce(lambda total, c: total + c, elf))

elf_cals = sorted(elf_cals)
print(elf_cals[-1])
print(elf_cals[-1] + elf_cals[-2] + elf_cals[-3])