type Point = Vec<f64>;

/// Mathematical factorial function
pub fn factorial(x: u32) -> u32 {
	// Using a loop instead of recursion, suck it mathematicians
	let mut value = 1;

	for i in 2..=x {
		value *= i;
	}

	value
}

/// Calculates a value given information for the Bernstein polynomials being used
/// this is suuuper slow and should be replaced. For now I just want pretty graphs.
pub fn bernstein_polynomials(control_point: u32, num_control_points: u32, u: f64) -> f64 {
	let n_fac = factorial(num_control_points) as f64;
	let i_fac = factorial(control_point) as f64;
	let n_min_i = num_control_points - control_point;
	let n_min_i_fac = factorial(n_min_i) as f64;
	let u_i = u.powi(control_point as i32);
	let inverse = (1.0 - u).powi(n_min_i as i32);

	n_fac * u_i * inverse / (i_fac * n_min_i_fac)
}

// Calculates the Bezier curve at this u value
pub fn calculate_bezier_curve(control_points: &Vec<Point>, u: f64) -> Point {
	let num_points = control_points.len() - 1;
	let mut weights = Vec::new();

	for point in 0..num_points {
		let weight = bernstein_polynomials(point as u32, num_points as u32, u);
		weights.push(weight);
	}

	let mut x = 0.0;
	let mut y = 0.0;

	for (i, point) in control_points.iter().enumerate() {
		let weight = bernstein_polynomials(i as u32, num_points as u32, u);
		x += weight * point[0];
		y += weight * point[1];
	}

	vec![x, y]
}

pub fn calculate_bezier_points(control_points: &Vec<Point>, num_points: u32) -> Vec<Point> {
	let delta = 1.0 / num_points as f64;
	let mut points = Vec::new();

	// Calculate each point
	for i in 0..=num_points {
		let u = i as f64 * delta;
		let point = calculate_bezier_curve(control_points, u);
		points.push(point);
	}

	points
}

pub fn calculate_fn_points<F>(num_points: u32, func: F, domain_min: f64, domain_max: f64)
-> Vec<Point>
where F: Fn(f64) -> f64 {
	let delta = (domain_max - domain_min) / num_points as f64;
	let mut points = Vec::new();

	// Calculate each point
	for i in 0..=num_points {
		let x = i as f64 * delta + domain_min;
		let point = func(x);
		points.push(vec![x, point]);
	}

	points
}

pub fn linear_interpolate(x_0: f64, y_0: f64, x_1: f64, y_1: f64, x: f64) -> f64 {
	((y_1-y_0)/(x_1-x_0)*(x-x_0)) + y_0
}
