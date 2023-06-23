use charts::{Chart, ScaleLinear, MarkerType, LineSeriesView, Color};

type Point = Vec<f64>;

fn main() -> Result<(), String> {
	let num_points = 50;

	let control_points = vec![
		vec![0.0, 0.0],
		vec![60.0, 70.0],
		vec![140.0, 30.0],
		vec![200.0, 100.0],
	];

	let domain_min = 0.0;
	let domain_max = 200.0;

	let points = calculate_fn_points(num_points, sin, domain_min, domain_max);
	let func = points_to_graph_data(&points);

	let points = calculate_bezier_points(&control_points, num_points);
	let spline = points_to_graph_data(&points);

	let control_point_data = points_to_graph_data(&control_points);
	show_spline_vs_fn(2500, 1300, &spline, &control_point_data, &func)
}

fn show_spline_vs_fn(width: isize, height: isize, spline: &Vec<(f32, f32)>, control_point_data: &Vec<(f32, f32)>, func: &Vec<(f32, f32)>) -> Result<(), String> {
	let (top, right, bottom, left) = (90, 40, 50, 60);

	// Set up axes
	let x = ScaleLinear::new()
		.set_domain(vec![0.0, 200.0])
		.set_range(vec![0, width - left - right]);

	let y = ScaleLinear::new()
		.set_domain(vec![0.0, 100.0])
		.set_range(vec![height - top - bottom, 0]);

	// Create spline points
	let spline_view = LineSeriesView::new()
		.set_x_scale(&x)
		.set_y_scale(&y)
		.set_marker_type(MarkerType::Circle)
		.set_label_visibility(false)
		.set_colors(Color::from_vec_of_hex_strings(vec!["#69dead"]))
		.load_data(spline).unwrap();

	// Create control points
	let control_view = LineSeriesView::new()
		.set_x_scale(&x)
		.set_y_scale(&y)
		.set_marker_type(MarkerType::X)
		.set_label_visibility(false)
		.set_colors(Color::from_vec_of_hex_strings(vec!["#dead69"]))
		.load_data(control_point_data).unwrap();

	// Create function points
	let func_view = LineSeriesView::new()
		.set_x_scale(&x)
		.set_y_scale(&y)
		.set_marker_type(MarkerType::Square)
		.set_label_visibility(false)
		.set_colors(Color::from_vec_of_hex_strings(vec!["#ad69de"]))
		.load_data(func).unwrap();

	// Generate and save the chart.
	Chart::new()
		.set_width(width)
		.set_height(height)
		.set_margins(top, right, bottom, left)
		.add_view(&spline_view)
		.add_view(&control_view)
		.add_view(&func_view)
		.add_axis_bottom(&x)
		.add_axis_left(&y)
		.save("chart.svg")
}

/// Mathematical factorial function
fn factorial(x: u32) -> u32 {
	// Using a loop instead of recursion, suck it mathematicians
	let mut value = 1;

	for i in 2..=x {
		value *= i;
	}

	value
}

/// Calculates a value given information for the Bernstein polynomials being used
/// this is suuuper slow and should be replaced. For now I just want pretty graphs.
fn bernstein_polynomials(control_point: u32, num_control_points: u32, u: f64) -> f64 {
	let n_fac = factorial(num_control_points) as f64;
	let i_fac = factorial(control_point) as f64;
	let n_min_i = num_control_points - control_point;
	let n_min_i_fac = factorial(n_min_i) as f64;
	let u_i = u.powi(control_point as i32);
	let inverse = (1.0 - u).powi(n_min_i as i32);

	n_fac * u_i * inverse / (i_fac * n_min_i_fac)
}

// Calculates the Bezier curve at this u value
fn calculate_bezier_curve(control_points: &Vec<Point>, u: f64) -> Point {
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

fn calculate_bezier_points(control_points: &Vec<Point>, num_points: u32) -> Vec<Point> {
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

fn points_to_graph_data(points: &Vec<Point>) -> Vec<(f32, f32)> {
	let mut data = Vec::new();

	for point in points {
		let datum = (point[0] as f32, point[1] as f32);
		data.push(datum);
	}

	data
}

fn calculate_fn_points<F>(num_points: u32, func: F, domain_min: f64, domain_max: f64)
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

fn sin(x: f64) -> f64 {
	50.0 * (x/25.0).sin() + 50.0
}
