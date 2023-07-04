use anyhow::{anyhow, Result};
use float_cmp::approx_eq;

use super::control_point::ControlPoint;

/// Represents a non-uniform rational B-Spline
pub struct BSpline {
	control_points: Vec<ControlPoint>,
	knots: Vec<f32>,
}

impl BSpline {
	/// Create a new spline
	pub fn new(control_points: Vec<ControlPoint>, knots: Vec<f32>) -> Result<Self> {
		check_dimensions(&control_points)?;

		Ok(BSpline {control_points, knots})
	}

	/// Find the values of each basis function at u
	fn basis_functions(&self, u: f32) -> Vec<f32> {
		let degree = self.control_points.len();
		let knots = &self.knots;

		let span = find_span(degree, knots, u);

		basis_functions(degree, knots, span, u)
	}

	/// Calculates the point at a certain u value
	pub fn calulate_point(&self, u: f32) -> Result<Vec<f32>> {
		let basis_values = self.basis_functions(u);

		let point = Vec::new();

		for i in 0..basis_values.len() {
			let control = self.control_points.get(i);
		}

		Ok(point)
	}

	/// The number of dimensions this spline works in
	pub fn num_dimensions(&self) -> Result<usize> {
		let control_0 = self.control_points
			.get(0)
			.ok_or(anyhow!("Can't find dimensions of spline with no control points"))?;

		Ok(control_0.position.len())
	}
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
fn basis_functions(degree: usize, knots: &Vec<f32>, span: usize, u: f32) -> Vec<f32> {
	let mut outputs = Vec::new();
	outputs.push(1.0);
	let mut left = Vec::new();
	left.push(0.0);
	let mut right = Vec::new();
	right.push(0.0);

	for i in 1..=degree {
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


/// Checks to make sure that every control point has the same number of dimensions
fn check_dimensions(control_points: &Vec<ControlPoint>) -> Result<()> {
	let dimension;

	match control_points.get(0) {
		Some(point) => dimension = point.position.len(),
		None => return Err(anyhow!("B-spline has no control points"))
	}

	for point in control_points {
		if point.position.len() != dimension {
			return Err(anyhow!("B-spline control points are not all the same dimension"))
		}
	}

	Ok(())
}

/// Finds which knot span a certain u value is in
///
/// This algorithm is ripped right from the NURBS book
///
/// * `degree`	- The polynomial degree of the basis functions
/// * `knots`	- A vector containing the u value of each knot
/// * 'u'		- The u value
fn find_span(degree: usize, knots: &Vec<f32>, u: f32) -> usize {
	let n = knots.len();

	// Cover the edge case at the end
	if approx_eq!(f32, u, knots[n + 1]) {
		return n
	}

	// Binary search to find it
	let mut low = degree;
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
