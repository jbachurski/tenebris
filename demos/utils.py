import math
import random


def norm(i, j):
    return math.sqrt((i**2) + (j**2))


def poisson_disk_sample(elems, dist, n):
    res = []
    for _ in range(n):
        ai, aj = random.choice(elems)
        while not all(norm(i - ai, j - aj) >= dist for (i, j) in res):
            ai, aj = random.choice(elems)
        res.append((ai, aj))
    return res
