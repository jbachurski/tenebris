use bevy::prelude::*;

use crate::tilesim::Simulator;

pub fn tile_position_rand(tile_position: UVec2) -> usize {
	((31 * tile_position.x + 37 * tile_position.y + 1337) ^ (tile_position.x * 7 + tile_position.y * 11)) as usize
}

#[rustfmt::skip]
pub fn tile_atlas_index(simulator: &Simulator, tile_position: UVec2) -> usize {
	let f = |dx: i32, dy: i32| -> bool {
		let xx = ((tile_position.x as i32) + dx) as usize;
		let yy = ((tile_position.y as i32) + dy) as usize;
		simulator
			.grid
			.is_wall
			.get(xx)
			.map_or(false, |row| *row.get(yy).unwrap_or(&false))
	};
	let v = tile_position_rand(tile_position);

	const X: i32 = 1; // wall
	const T: i32 = 0; // any
	const O: i32 = -1; // open

	// pattern center at (1, 1)
	let patterns = [
		(
			[ // up
				[T, O, T],
				[X, X, X],
				[T, T, T],
				[T, T, T]
			],
			613 + v%4
		),
		(
			[ // up left
				[T, O, T],
				[O, X, X],
				[T, X, T],
				[T, T, T]
			],
			2
		),
		(
			[ // up right
				[T, O, T],
				[X, X, O],
				[T, X, T],
				[T, T, T]
			],
			7
		),
		(
			[ // right
				[T, X, T],
				[X, X, O],
				[T, X, T],
				[T, T, T]
			],
			58 + 51 * (v%6)
		),
		(
			[ // left
				[T, X, T],
				[O, X, X],
				[T, X, T],
				[T, T, T]
			],
			53 + 51 * (v%6)
		),
		(
			[ // down low
				[T, X, T],
				[T, O, T],
				[T, T, T],
				[T, T, T]
			],
			562 + v%4
		),
		(
			[ // down mid
				[T, X, T],
				[T, X, T],
				[T, O, T],
				[T, T, T]
			],
			511 + v%4
		),
		(
			[ // down high
				[T, X, T],
				[T, X, T],
				[T, X, T],
				[T, O, T]
			],
			460 + v%4
		),
		(
			[ // mid
				[X, X, X],
				[X, X, X],
				[X, X, X],
				[T, T, T]
			],
			54 + v%4 + 51*((v/4)%5)
		),
		(
			[ // generic open
				[T, T, T],
				[T, O, T],
				[T, T, T],
				[T, T, T]
			],
			1775 + v%3 + 51*((v/3)%3)
		),
		(
			[ // generic wall
				[T, T, T],
				[T, X, T],
				[T, T, T],
				[T, T, T]
			],
			208
		)
	];

	for (pattern, target) in patterns {
		let mut ok = true;
		for (dy, row) in (-1..=2).zip(pattern) {
			for (dx, a) in (-1..=1).zip(row) {
				if !((f(dx, -dy) && a >= 0) || (!f(dx, -dy) && a <= 0)) {
					ok = false;
					break;
				}
			}
		}
		if ok { return target; }
	};
	panic!();
}
