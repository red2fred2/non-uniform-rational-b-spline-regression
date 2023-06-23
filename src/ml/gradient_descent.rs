type Point = Vec<f64>;

pub fn gradient_descent<F>(start_point: Point, end_point: Point, test_fn: F, num_controls: u8)
-> Vec<f64>
where F: Fn(f64) -> f64 {
	vec![]
}