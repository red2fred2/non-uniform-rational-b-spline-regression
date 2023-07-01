use super::initialization::init_data;
use super::neighbors::neighbors;

type Point = Vec<f64>;

pub fn simulated_annealing<F>(start_point: &Point, end_point: &Point, test_fn: &F, num_controls: u8)
-> Vec<f64>
where F: Fn(f64) -> f64 {
	let start_temp = 1.0;
	let end_temp = 0.001;
	let alpha = 0.9;

	let mut temperature = start_temp;
	let mut data = init_data(start_point, end_point, num_controls);

	let samples = 100;

	while temperature > end_temp {
		data = step_data(&data, start_point, end_point, test_fn, samples, temperature);
		temperature *= alpha;
	}

	data
}

fn step_data<F>(data: &Vec<f64>, start_point: &Point, end_point: &Point, test_fn: &F, samples: u32, temperature: f32) -> Vec<f64>
where F: Fn(f64) -> f64 {
	let neighbors = neighbors(data);



	neighbors[0].clone()
}
