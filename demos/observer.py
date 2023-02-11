import math
import random

import pygame
from campfire import PygameFrontendWithCampfires, RadarSimulatorWithCampfires

# Arrow keys to move around, press return to place campfires


class ObserverFrontend(PygameFrontendWithCampfires):
    def __init__(self, *args, fog_of_war=True, **kwargs):
        super().__init__(*args, **kwargs)
        self.fog_of_war = fog_of_war

    def handle(self, ev):
        # Override the manual campfire click-placing
        pass

    def get_colour(self, i, j):
        if (i, j) == self.sim.ci:
            return (255, 0, 0)

        r, g, b = super().get_colour(i, j)
        if (i, j) in self.sim.available_cells:
            b *= 0.8
            if (
                self.sim.norm(i - self.sim.ci[0], j - self.sim.ci[1])
                < self.sim.innerrad
            ):
                g *= 0.8
        elif self.fog_of_war:
            return (150, 150, 150)
        return (r, g, b)

    def should_force_update(self, i, j):
        return True

    def step(self):
        super().step()
        self.move()

    def move(self):
        pressed = pygame.key.get_pressed()
        i, j = self.sim.ci
        if pressed[pygame.K_UP]:
            self.sim.ci = (i - 1, j)
        if pressed[pygame.K_DOWN]:
            self.sim.ci = (i + 1, j)
        if pressed[pygame.K_LEFT]:
            self.sim.ci = (i, j - 1)
        if pressed[pygame.K_RIGHT]:
            self.sim.ci = (i, j + 1)
        if pressed[pygame.K_RETURN]:
            self.sim.toggle_campfire(*self.sim.ci)
        self.update_cell(i, j)
        self.update_cell(*self.sim.ci)


class ObserverSimulator(RadarSimulatorWithCampfires):
    def __init__(self, *args, innerrad=10, outerrad=15, half_life=10, **kwargs):
        assert outerrad >= innerrad
        super().__init__(*args, **kwargs)
        self.outerrad = outerrad
        self.ci = (self.width // 2, self.width // 2)
        self.available_cells = set()
        self.despawn_prob = 1 - math.pow(0.5, 1 / half_life)

        # Initialise CA without worrying about innerrad for now
        self.innerrad = 0
        for _ in range(10):
            self.step()
        self.innerrad = innerrad

    def step(self):
        super().step()
        for i, j in list(self.available_cells):
            dist = self.norm(i - self.ci[0], j - self.ci[1])
            if dist > self.outerrad and random.random() < self.despawn_prob:
                # Despawn
                if not self.protected(i, j):
                    self.available_cells.remove((i, j))
                    self.grid[i][j] = self.calc_new_cell(i, j)

    def calc(self, i, j):
        dist = self.norm(i - self.ci[0], j - self.ci[1])
        if dist < self.outerrad:
            # This cell exists!!!
            self.available_cells.add((i, j))
        if self.innerrad <= dist <= self.outerrad:
            return super().calc(i, j)
        return self.grid[i][j]


if __name__ == "__main__":
    N = 80
    s = ObserverSimulator(
        2 * N + 2,
        2,
        6,
        radii=(0, 9 * N / 10),
        stepangle=-1,
        campfire_radius=15,
        innerrad=15,
        outerrad=20,
        half_life=2,
    )
    pf = ObserverFrontend(s, pxsz=6, fog_of_war=True)

    pf.run()
