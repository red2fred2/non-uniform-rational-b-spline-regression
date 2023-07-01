/// Returns the neighbors of the current settings
pub fn neighbors(data: &Vec<f64>) -> Vec<Vec<f64>> {
	let delta = 1.0;
	let mut neighbors = Vec::new();
	neighbors.push(data.clone());

	// Try turning each knob and figure out which was best
	for i in 0..data.len() {
		// Step positive
		let mut test_data = data.clone();
		test_data[i] += delta;
		neighbors.push(test_data);


		// Step negative
		let mut test_data = data.clone();
		test_data[i] -= delta;
		neighbors.push(test_data);
	}

	neighbors
}
