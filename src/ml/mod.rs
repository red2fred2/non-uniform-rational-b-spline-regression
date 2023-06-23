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
