mod fitness;
mod gradient_descent;
mod initialization;
mod neighbors;
mod simulated_annealing;

use initialization::build_control_points;
use simulated_annealing::simulated_annealing;

type Point = Vec<f64>;

/// Tries to match a spline to a function over a set domain
pub fn regress_spline<F>(func: &F, domain_min: f64, domain_max: f64) -> Vec<Point>
where F: Fn(f64) -> f64 {
	// Set start and end points
	let start_point = vec![domain_min, func(domain_min)];
	let end_point = vec![domain_max, func(domain_max)];

	let data = simulated_annealing(&start_point, &end_point, func, 4);
	build_control_points(&data, &start_point, &end_point)
}
