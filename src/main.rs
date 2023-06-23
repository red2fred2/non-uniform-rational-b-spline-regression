mod graph;
mod ml;
mod math;

fn main() -> Result<(), String> {
	let domain_min = 0.0;
	let domain_max = 200.0;

	let control_points = vec![
		vec![0.0, 50.0],
		vec![200.0, 99.46791233116909],
	];

	// Regress to best spline
	let samples = 50;
	let error = ml::fitness::fitness(&control_points, fun, samples);

	println!("Error: {error}");

	// Generate graphs
	let num_points = 50;

	let points = math::calculate_fn_points(num_points, fun, domain_min, domain_max);
	let func = graph::points_to_graph_data(&points);

	let points = math::calculate_bezier_points(&control_points, num_points);
	let spline = graph::points_to_graph_data(&points);

	let control_point_data = graph::points_to_graph_data(&control_points);
	graph::show_spline_vs_fn(&spline, &control_point_data, &func)?;

	Ok(())
}

fn fun(x: f64) -> f64 {
	50.0 * (x/25.0).sin() + 50.0
}
