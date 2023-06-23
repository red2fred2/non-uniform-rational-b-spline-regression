pub mod fitness;

type Point = Vec<f64>;

pub fn regress_spline<F>(func: F, domain_min: f64, domain_max: f64) -> Vec<Point>
where F: Fn(f64) -> f64 {
	// Set start and end points
	let start_point = vec![domain_min, func(domain_min)];
	let end_point = vec![domain_max, func(domain_max)];


	let samples = 15;
	let error = fitness::fitness(&vec![start_point.clone(), end_point.clone()], func, samples);

	println!("Error: {error}");

	vec![
		start_point,
		end_point
	]
}
