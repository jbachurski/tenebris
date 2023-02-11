from observer import ObserverFrontend, ObserverSimulator
from utils import norm, poisson_disk_sample


class StructureFrontend(ObserverFrontend):
    def get_colour(self, i, j):
        if (i, j) in self.sim.structures:
            return (0, 0, 255)
        return super().get_colour(i, j)


class StructuresSimulator(ObserverSimulator):
    def __init__(
        self,
        *args,
        n_structures=10,
        structure_distance=20,
        structure_radius=3,
        **kwargs
    ):
        super().__init__(*args, **kwargs)
        self.n_structures = n_structures
        self.structure_distance = structure_distance
        self.structure_radius = structure_radius
        self.structures = []

    def post_init(self):
        structure_choices = [
            (i, j)
            for i in range(self.width)
            for j in range(self.width)
            if self.grid[i][j] == 0
        ]

        # Poisson disk sampling
        self.structures = poisson_disk_sample(
            structure_choices, self.structure_distance, self.n_structures
        )

        for ai, aj in self.structures:
            for i in range(self.width):
                for j in range(self.width):
                    if norm(i - ai, j - aj) <= self.structure_radius:
                        self.grid[i][j] = 0

        super().post_init()

    def protected(self, i, j):
        return any(
            norm(i - ai, j - aj) <= self.structure_radius
            for (ai, aj) in self.structures
        )


if __name__ == "__main__":
    N = 80
    s = StructuresSimulator(
        2 * N + 2,
        3,
        6,
        weights=[1, 1.3],
        radii=(0, 9 * N / 10),
        stepangle=-1,
        campfire_radius=15,
        innerrad=15,
        outerrad=20,
        half_life=2,
        n_structures=10,
        structure_distance=20,
        structure_radius=5,
    )
    pf = StructureFrontend(s, pxsz=6, fog_of_war=False)

    pf.run()
