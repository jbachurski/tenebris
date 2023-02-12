pub fn lerp(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
	((x - x1) * y2 + (x2 - x) * y1) / (x2 - x1)
}
