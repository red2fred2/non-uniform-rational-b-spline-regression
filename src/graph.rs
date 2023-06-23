use charts::{Chart, ScaleLinear, MarkerType, LineSeriesView, Color};

pub fn show_spline_vs_fn(spline: &Vec<(f32, f32)>, control_point_data: &Vec<(f32, f32)>, func: &Vec<(f32, f32)>) -> Result<(), String> {
	let width = 2500;
	let height = 1300;

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