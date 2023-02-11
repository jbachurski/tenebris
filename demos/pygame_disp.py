import pygame
import time
import copy
import math

pygame.init()

class PygameFrontend:
    def __init__(self, sim, pxsz=2):
        self.sim = sim
        self.pxsz = pxsz
        self.screen = pygame.display.set_mode((sim.width * pxsz, sim.width * pxsz))
        self.steps = 0

    def run(self):
        while True:
            for ev in pygame.event.get():
                if ev.type == pygame.QUIT:
                    return
                self.handle(ev)

            prev_grid = copy.deepcopy(self.sim.grid)
            self.sim.step()
            
            for i in range(self.sim.width):
                for j in range(self.sim.width):
                    if self.sim.grid[i][j] != prev_grid[i][j] or self.should_force_update(i, j):
                        col = self.get_colour(i, j)
                        if self.pxsz == 1:
                            self.screen.set_at((j, i), col)
                        else:
                            pygame.draw.rect(self.screen, col, [j*self.pxsz, i*self.pxsz, self.pxsz, self.pxsz])

            pygame.display.flip()
            self.steps += 1
            time.sleep(0.1)
    
    def should_force_update(self, i, j):
        return self.steps == 0

    def get_colour(self, i, j):
        state = self.sim.grid[i][j]
        return [(0, 0, 0), (255, 255, 255)][::-1][state]

    def translate(self, xy):
        return int(xy[0] / self.pxsz), int(xy[1] / self.pxsz)

    def handle(self, ev):
        return

if __name__ == "__main__":
    from radar import RadarSimulator
    N = 100
    s = RadarSimulator(2*N+2, 2, 6, radii=(0, 9*N/10), stepangle=0.5)
    pf = PygameFrontend(s, pxsz=3)

    pf.run()
