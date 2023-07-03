use float_cmp::approx_eq;

/// Finds which knot span a certain u value is in
///
/// This algorithm is ripped right from the NURBS book
///
/// * `degree`	- The polynomial degree of the basis functions
/// * `knots`	- A vector containing the u value of each knot
/// * 'u'		- The u value
fn find_span(degree: u8, knots: Vec<f32>, u: f32) -> usize {
	let n = knots.len();

	// Cover the edge case at the end
	if approx_eq!(f32, u, knots[n + 1]) {
		return n
	}

	// Binary search to find it
	let mut low = degree as usize;
	let mut high = n + 1;
	let mut mid = (low + high) / 2;

	while u < knots[mid] || u >= knots[mid + 1] {
		if u < knots[mid] {
			high = mid;
		} else {
			low = mid;
		}

		mid = (low + high) / 2;
	}

	mid
}

/// Find the values of each basis function at u
///
/// This algorithm is ripped right from the NURBS book, it may need to be
/// modified to work for tensor product surfaces.
///
/// * `degree`	- The polynomial degree of the basis functions
/// * `knots`	- A vector containing the u value of each knot
/// * `span`	- Which span the u value is in
/// * 'u'		- The u value
fn basis_functions(degree: u8, knots: Vec<f32>, span: usize, u: f32) -> Vec<f32> {
	let mut outputs = Vec::new();
	outputs.push(1.0);
	let mut left = Vec::new();
	left.push(0.0);
	let mut right = Vec::new();
	right.push(0.0);

	for i in 1..=degree as usize {
		let left_side = u - knots[span + 1 - i];
		left.push(left_side);

		let right_side = knots[span + i] - u;
		right.push(right_side);

		let mut saved = 0.0;

		for j in 0..i {
			let temp = outputs[j] / (right[j + 1] + left[i - j]);
			outputs[j] = saved + right[j + 1] * temp;
			saved = left[i - j] * temp;
		}

		outputs[i] = saved;
	}

	outputs
}
