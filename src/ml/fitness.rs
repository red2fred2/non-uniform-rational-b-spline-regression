type Point = Vec<f64>;

/// Core fitness function
/// Measures how much the current attempt sucks
pub fn fitness<F>(data: &Vec<f64>, start_point: &Point, end_point: &Point, test_fn: &F, samples: u32) -> f32
where F: Fn(f64) -> f64 {
	let delta = 1.0 / samples as f64;
	let control_points = super::initialization::build_control_points(data, start_point, end_point);

	let mut mean_squared_error: f32 = 0.0;

	for i in 0..=samples {
		// Calculate curve
		let u = i as f64 * delta;
		let point = crate::math::calculate_bezier_curve(&control_points, u);
		let y_curve = point[1];

		// Calculate fn
		let x = point[0];
		let y_fn = test_fn(x);

		// Calculate error
		mean_squared_error += ((y_fn - y_curve) * (y_fn - y_curve)) as f32;
	}

	mean_squared_error.sqrt()
}
