mod graph;
mod ml;
mod math;

fn main() -> Result<(), String> {
	// Regress to best spline
	let domain_min = 0.0;
	let domain_max = 200.0;

	let control_points = ml::regress_spline(fun, domain_min, domain_max);

	// Generate graphs
	let num_points = 150;

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
