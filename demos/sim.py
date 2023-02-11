import math
import random

def mk_grid(width):
    return [[0 for _ in range(width)] for _ in range(width)]

class Simulator:
    def __init__(self, width, k, b, radii=(1, 38)):
        self.width = width
        self.k = k
        self.b = b
        self.radii = radii
        self.grid = [[self.calc_new_cell(i, j) for j in range(self.width)] for i in range(self.width)]

    def step(self):
        self.grid = [[self.calc(i, j) for j in range(self.width)] for i in range(self.width)]
                
    def calc(self, i, j):
        tot = 0
        for dx in range(-1, 2):
            if 0 <= i + dx < self.width:
                for dy in range(-1, 2):
                    if 0 <= j + dy < self.width and (dx != 0 or dy != 0):
                        tot += self.grid[i+dx][j+dy]

        if tot <= self.k:
            return 0
        if tot >= self.b:
            return 1
        return self.grid[i][j]

    def norm(self, i, j):
        return math.sqrt(i**2 + j**2)

    def calc_new_cell(self, i, j):
        dist = self.norm(i - self.width // 2, j - self.width // 2)
        if dist < self.radii[0] or dist > self.radii[1]:
            return 1
        else:
            return random.randint(0, 1)

    def output(self):
        for i in range(self.width):
            for j in range(self.width):
                print(".#"[self.grid[i][j]], end="")
            print()

if __name__ == "__main__":
    s = Simulator(80, 3, 6)
    x = ""
    while x == "":
        s.output()
        x = input()
        s.step()
