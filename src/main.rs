mod graph;
mod math;

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

	let points = math::calculate_fn_points(num_points, sin, domain_min, domain_max);
	let func = graph::points_to_graph_data(&points);

	let points = math::calculate_bezier_points(&control_points, num_points);
	let spline = graph::points_to_graph_data(&points);

	let control_point_data = graph::points_to_graph_data(&control_points);
	graph::show_spline_vs_fn(&spline, &control_point_data, &func)
}

fn sin(x: f64) -> f64 {
	50.0 * (x/25.0).sin() + 50.0
}
