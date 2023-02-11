import pygame
from campfire import PygameFrontendWithCampfires, RadarSimulatorWithCampfires


class ObserverFrontend(PygameFrontendWithCampfires):
    def get_colour(self, i, j):
        if (i, j) == self.sim.ci:
            return (255, 0, 0)
        return super().get_colour(i, j)

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
    def __init__(self, *args, innerrad=10, outerrad=15, outestrad=18, **kwargs):
        assert outerrad >= innerrad
        super().__init__(*args, **kwargs)
        self.outerrad = outerrad
        self.outestrad = outestrad
        self.ci = (self.width // 2, self.width // 2)

        # Initialise CA without worrying about innerrad for now
        self.innerrad = 0
        for _ in range(10):
            self.step()
        self.innerrad = innerrad

    def calc(self, i, j):
        dist = self.norm(i - self.ci[0], j - self.ci[1])
        if self.innerrad <= dist <= self.outerrad:
            return super().calc(i, j)
        if self.outerrad < dist <= self.outestrad and not self.protected(i, j):
            return self.calc_new_cell(i, j)
        return self.grid[i][j]


if __name__ == "__main__":
    N = 100
    s = ObserverSimulator(
        2 * N + 2,
        2,
        6,
        radii=(0, 9 * N / 10),
        stepangle=-1,
        campfire_radius=10,
        innerrad=20,
        outerrad=25,
        outestrad=27,
    )
    pf = ObserverFrontend(s, pxsz=3)

    pf.run()
