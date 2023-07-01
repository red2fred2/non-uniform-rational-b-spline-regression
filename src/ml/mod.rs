mod fitness;
mod gradient_descent;

type Point = Vec<f64>;

pub fn regress_spline<F>(func: &F, domain_min: f64, domain_max: f64) -> Vec<Point>
where F: Fn(f64) -> f64 {
	// Set start and end points
	let start_point = vec![domain_min, func(domain_min)];
	let end_point = vec![domain_max, func(domain_max)];

	let data = gradient_descent::gradient_descent(&start_point, &end_point, func, 4);
	build_control_points(&data, &start_point, &end_point)
}

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
