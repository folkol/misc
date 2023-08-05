import random
from string import ascii_lowercase as alphabet
import sys

N = 1_000_000

random.seed(1337)
print('Generating test input', file=sys.stderr)

for n in range(N):
    l = random.randint(0, 500)
    print('x' * l)
