use bevy::prelude::UVec2;
use rand::*;

pub fn _todo_remove_norm(i: f32, j: f32) -> f32 {
	(i.powf(2.0) + j.powf(2.0)).sqrt()
}

pub fn poisson_disk_sample(elems: &Vec<UVec2>, dist: f32, k: u32) -> Vec<UVec2> {
	let n = elems.len();
	let mut res = Vec::default();
	for _ in 0..k {
		let mut tries = 1000;
		let mut a = elems[thread_rng().gen_range(0..n)];
		while !res.iter().all(|uv: &UVec2| (*uv).as_vec2().distance(a.as_vec2()) >= dist) && tries > 0 {
			a = elems[thread_rng().gen_range(0..n)];
			tries -= 1;
		}
		if tries <= 0 {
			panic!("Tried to Poisson disk sample but failed after 1000 tries. Did you set the distance between points to be too large?");
		}
		res.push(a);
	}
	return res;
}