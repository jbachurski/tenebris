use bevy::{prelude::*, utils::*};
use rand::*;

use crate::{structures::*, tiles::*, utils::*};

#[derive(Resource)]
pub struct SimulatorTimer(pub Timer);

#[derive(Resource)]
pub struct Simulator {
	width: u32,
	ca_params: (u32, u32),
	radii: (u32, u32),
	weights: (u32, u32),
	campfire_radius: u32,
	reality_params: (u32, u32),
	despawn_prob: f32,
	n_structures: u32,
	structure_dist: u32,
	structure_radius: u32,
	pub grid: TileManager,
}

impl Simulator {
	pub fn new(
		width: u32,
		ca_params: (u32, u32),
		radii: (u32, u32),
		weights: (u32, u32),
		campfire_radius: u32,
		reality_params: (u32, u32),
		half_life: u32,
		n_structures: u32,
		structure_dist: u32,
		structure_radius: u32,
	) -> Simulator {
		Self {
			width: width,
			ca_params: ca_params,
			radii: radii,
			weights: weights,
			campfire_radius: campfire_radius,
			reality_params: reality_params,
			despawn_prob: 1.0 - 0.5_f32.powf(1.0 / half_life as f32),
			n_structures: n_structures,
			structure_dist: structure_dist,
			structure_radius: structure_radius,
			grid: TileManager::default(),
		}
	}

	pub fn post_init(&mut self) {
		// Generate grid
		for i in 0..self.width {
			for j in 0..self.width {
				self.grid.is_wall[i as usize][j as usize] = self.calc_new_cell(UVec2::new(i, j));
			}
		}

		// Spawn structures
		let structure_choices = (1usize..self.width as usize)
			.flat_map(|i| (1usize..self.width as usize).map(move |j| (i, j)))
			.filter(|(i, j)| {
				!self.grid.is_wall[*i][*j]
					&& Vec2::new(*i as f32, *j as f32).distance(self.world_center().as_vec2()) > self.radii.0 as f32
			})
			.map(|(i, j)| UVec2::new(i as u32, j as u32))
			.collect();

		self.grid.structures = HashMap::default();

		for pos in poisson_disk_sample(&structure_choices, self.structure_dist as f32, self.n_structures).iter() {
			self.grid.structures.insert(*pos, StructureType::Unspawned);
		}

		for i in 0..self.width {
			for j in 0..self.width {
				let loc = UVec2::new(i, j);
				if self
					.grid
					.structures
					.iter()
					.any(|(sv, _)| sv.as_vec2().distance(loc.as_vec2()) <= self.structure_radius as f32)
				{
					// Space near structures should be reserved
					self.grid.is_wall[i as usize][j as usize] = false;
				}
			}
		}
	}

	pub fn step(&mut self, player_pos: UVec2) {
		// Step the cellular automaton \o/
		for i in 0..self.width {
			for j in 0..self.width {
				self.grid.is_wall[i as usize][j as usize] = self.calc(UVec2::new(i, j), player_pos);
			}
		}
		// Check out all of the available cells if any should be despawned
		let mut to_remove = Vec::default();
		let (_, outerrad) = self.reality_params;
		let outerrad = outerrad as f32;
		for ac in self.grid.reality_bubble.iter() {
			let dist = ac.as_vec2().distance(player_pos.as_vec2());
			if dist > outerrad as f32 && rand::random::<f32>() < self.despawn_prob {
				// Despawn
				if !self.cannot_forget(*ac) {
					to_remove.push(ac.clone());
				}
			}
		}
		for ac in to_remove {
			let (i, j) = ac.into();
			self.grid.reality_bubble.remove(&ac);
			if !self.protected(ac) {
				self.grid.is_wall[i as usize][j as usize] = self.calc_new_cell(ac);
			}
		}
	}

	fn calc(&mut self, loc: UVec2, player_pos: UVec2) -> bool {
		let (i, j) = loc.into();
		let (k, b) = self.ca_params;
		let (innerrad, outerrad) = self.reality_params;
		let innerrad = innerrad as f32;
		let outerrad = outerrad as f32;
		// Calculate distance from player
		let dist = loc.as_vec2().distance(player_pos.as_vec2());
		// Update available_cells
		if dist < outerrad && !self.grid.reality_bubble.contains(&loc) {
			self.grid.reality_bubble.insert(loc);
			// If structure, assign it a random value
			self.grid.structures.get_mut(&loc).map(|v| {
				*v = match thread_rng().gen_range(0..=1) {
					0 => StructureType::Remember,
					1 => StructureType::BewareSpider,
					_ => panic!(),
				};
			});
		}
		// If loc between inner_rad and outer_rad
		if innerrad <= dist && dist <= outerrad {
			if self.protected(loc) {
				return self.grid.is_wall[i as usize][j as usize];
			}
			// If not protected, perform GoL
			let mut tot = 0;
			for dx in -1..=1 {
				let (i, j) = (i as i32, j as i32);
				if 0 <= i + dx && i + dx < self.width as i32 {
					for dy in -1..=1 {
						if 0 <= j + dy
							&& j + dy < self.width as i32
							&& (dx != 0 || dy != 0) && self.grid.is_wall[(i + dx) as usize][(j + dy) as usize]
						{
							tot += 1;
						}
					}
				}
			}

			if tot <= k {
				return false;
			}
			if tot >= b {
				return true;
			}
			return self.grid.is_wall[i as usize][j as usize];
		}
		// For cells within inner_rad, return original cell
		return self.grid.is_wall[i as usize][j as usize];
	}

	fn cannot_forget(&self, loc: UVec2) -> bool {
		// Check if cell is within any campfires
		self.grid
			.campfires
			.iter()
			.any(|uv| uv.as_vec2().distance(loc.as_vec2()) < self.campfire_radius as f32)
	}

	fn protected(&self, loc: UVec2) -> bool {
		// Check if cannot forget
		if self.cannot_forget(loc) {
			return true;
		}
		// Check if cell is within structure_radius of other structures
		self.grid
			.structures
			.iter()
			.any(|(uv, _)| uv.as_vec2().distance(loc.as_vec2()) <= self.structure_radius as f32)
	}

	fn calc_new_cell(&self, loc: UVec2) -> bool {
		let dist = loc.as_vec2().distance(self.world_center().as_vec2());
		let (inner_bound, outer_bound) = self.radii;
		if dist < inner_bound as f32 {
			return false;
		}
		if dist > outer_bound as f32 {
			// If dist from centre is not within the two radii, return true
			return true;
		}
		// Else, return something random weighted 0, 1
		let (e, w) = self.weights;
		return rand::thread_rng().gen_ratio(w, e + w);
	}

	fn world_center(&self) -> UVec2 {
		return UVec2::new(self.width / 2, self.width / 2);
	}

	pub fn place_campfire(&mut self, loc: UVec2) {
		self.grid.campfires.insert(loc);
	}

	pub fn remove_campfire(&mut self, loc: UVec2) {
		self.grid.campfires.remove(&loc);
	}

	pub fn debug_output(&self) {
		let mut cs = 0;
		for i in 0..self.width {
			for j in 0..self.width {
				if self.grid.is_wall[i as usize][j as usize] {
					print!("#");
					cs += 1;
				} else {
					print!(".");
				}
			}
			println!();
		}
		println!("Checksum: {cs}")
	}
}
