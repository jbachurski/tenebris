import pygame
from pygame_disp import PygameFrontend
from radar import RadarSimulator

# Click screen to place down campfires


class PygameFrontendWithCampfires(PygameFrontend):
    def handle(self, ev):
        if ev.type == pygame.MOUSEBUTTONUP:
            j, i = pygame.mouse.get_pos()
            self.sim.toggle_campfire(*self.translate((i, j)))

    def get_colour(self, i, j):
        res = super().get_colour(i, j)
        if (i, j) in self.sim.campfires:
            return (0, 255, 0)
        return res

    def should_force_update(self, i, j):
        return super().should_force_update(i, j) or ((i, j) in self.sim.past_campfires)


class RadarSimulatorWithCampfires(RadarSimulator):
    def __init__(self, *args, campfire_radius=10, **kwargs):
        super().__init__(*args, **kwargs)
        self.campfire_radius = campfire_radius
        self.campfires = []
        self.past_campfires = []

    def toggle_campfire(self, i, j):
        if (i, j) in self.campfires:
            self.remove_campfire(i, j)
        else:
            self.place_campfire(i, j)

    def place_campfire(self, i, j):
        self.campfires.append((i, j))
        self.past_campfires.append((i, j))
        print("Placed campfire at", (i, j))

    def remove_campfire(self, i, j):
        self.campfires.remove((i, j))

    def calc(self, i, j):
        res = super().calc(i, j)
        for ci, cj in self.campfires:
            if self.norm(i - ci, j - cj) < self.campfire_radius:
                return self.grid[i][j]
        return res


if __name__ == "__main__":
    N = 50
    s = RadarSimulatorWithCampfires(
        2 * N + 2, 2, 6, radii=(0, 9 * N / 10), stepangle=0.5, campfire_radius=10
    )
    pf = PygameFrontendWithCampfires(s, pxsz=6)

    pf.run()
