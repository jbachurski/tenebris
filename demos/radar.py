import math
import time

from sim import Simulator


def within(alpha, beta, delta):
    return (alpha < beta < alpha + delta) or (alpha < beta + math.tau < alpha + delta)


class RadarSimulator(Simulator):
    def __init__(self, *args, stepangle=0.2, **kwargs):
        super().__init__(*args, **kwargs)
        # Use self.stepangle < 0 to turn off radar
        self.stepangle = stepangle
        self.cur = 0

    def step(self):
        self.cur = (self.cur + math.tau - self.stepangle) % math.tau
        super().step()

    def calc(self, i, j):
        prev_res = super().calc(i, j)
        if self.stepangle < 0:
            return prev_res
        x = i - self.width // 2
        y = j - self.width // 2
        arg = math.atan2(x, y) % math.tau
        if within(arg, self.cur, math.pi):
            return self.grid[i][j]
        if within(arg, self.cur, 5.5):
            return prev_res
        return self.calc_new_cell(i, j)


if __name__ == "__main__":
    s = RadarSimulator(80, 2, 6)

    while True:
        s.output()
        time.sleep(0.1)
        s.step()
