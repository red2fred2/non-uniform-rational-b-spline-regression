type Point = Vec<f64>;

pub fn simulated_annealing<F>(start_point: &Point, end_point: &Point, test_fn: &F, num_controls: u8)
-> Vec<f64>
where F: Fn(f64) -> f64 {
	unimplemented!()
}

fn step_data<F>(data: &Vec<f64>, start_point: &Point, end_point: &Point, test_fn: &F, samples: u32) -> Vec<f64>
where F: Fn(f64) -> f64 {
	unimplemented!()
}
