import sys

acc = 0
for line in sys.stdin:
    acc += (len(line) - 1)
    print(line, end='')
print(acc)

