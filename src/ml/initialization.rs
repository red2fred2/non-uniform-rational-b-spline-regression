use crate::math::linear_interpolate;

type Point = Vec<f64>;

/// Initialize control points for a spline
pub fn build_control_points(data: &Vec<f64>, start_point: &Point, end_point: &Point) -> Vec<Point> {
	let mut control_points = Vec::new();
	control_points.push(start_point.clone());

	for i in (0..data.len()).step_by(2) {
		let point = vec![data[i], data[i+1]];
		control_points.push(point);
	}

	control_points.push(end_point.clone());

	control_points
}

/// Initialize data to sit on a line between the first and last points
/// This may or may not be worth doing. More testing is needed.
pub fn init_data(start_point: &Point, end_point: &Point, num_controls: u8) -> Vec<f64> {
	let x_0 = start_point[0];
	let y_0 = start_point[1];
	let x_1 = end_point[0];
	let y_1 = end_point[1];

	let delta = (x_1 - x_0) / (num_controls + 1) as f64;
	let mut data = Vec::new();

	for i in 1..=num_controls {
		let x = x_0 + i as f64 * delta;
		data.push(x);

		let y = linear_interpolate(x_0, y_0, x_1, y_1, x);
		data.push(y)
	}

	data
}
