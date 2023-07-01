type Point = Vec<f64>;

pub fn gradient_descent<F>(start_point: &Point, end_point: &Point, test_fn: &F, num_controls: u8)
-> Vec<f64>
where F: Fn(f64) -> f64 {
	let mut data = super::init_data(start_point, end_point, num_controls);

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
