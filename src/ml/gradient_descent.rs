type Point = Vec<f64>;

pub fn gradient_descent<F>(start_point: &Point, end_point: &Point, test_fn: &F, num_controls: u8)
-> Vec<f64>
where F: Fn(f64) -> f64 {
	let mut data = init_data(start_point, end_point, num_controls);
	// let mut data = vec![100.0, 40.0];
	// let mut data = Vec::new();
	// for _ in 0..num_controls * 2 {
	// 	data.push(0.0)
	// }

	let samples = 100;

	for _ in 0..1000 {
		data = step_data(&data, start_point, end_point, test_fn, samples);
	}

	data
}

fn step_data<F>(data: &Vec<f64>, start_point: &Point, end_point: &Point, test_fn: &F, samples: u32) -> Vec<f64>
where F: Fn(f64) -> f64 {
	let delta = 1.0;

	// Get baseline
	let mut best_data = data.clone();
	let mut best_fitness = super::fitness::fitness(data, start_point, end_point, test_fn, samples);

	// Try turning each knob and figure out which was best
	for i in 0..data.len() {
		// Step positive
		let mut test_data = data.clone();
		test_data[i] += delta;
		let fitness = super::fitness::fitness(&test_data, start_point, end_point, test_fn, samples);

		if fitness < best_fitness {
			best_data = test_data;
			best_fitness = fitness;
		}

		// Step negative
		let mut test_data = data.clone();
		test_data[i] -= delta;
		let fitness = super::fitness::fitness(&test_data, start_point, end_point, test_fn, samples);

		if fitness < best_fitness {
			best_data = test_data;
			best_fitness = fitness;
		}
	}

	println!("Best fitness: {best_fitness}");
	best_data
}

fn init_data(start_point: &Point, end_point: &Point, num_controls: u8) -> Vec<f64> {
	let x_0 = start_point[0];
	let y_0 = start_point[1];
	let x_1 = end_point[0];
	let y_1 = end_point[1];

	let delta = (x_1 - x_0) / (num_controls + 1) as f64;
	let mut data = Vec::new();

	for i in 1..num_controls {
		let x = x_0 + i as f64 * delta;
		data.push(x);

		let y = crate::math::linear_interpolate(x_0, y_0, x_1, y_1, x);
		data.push(y)
	}

	data
}
