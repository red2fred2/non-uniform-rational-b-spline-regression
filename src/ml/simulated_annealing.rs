use crate::math::pick_from_weight_list;

use super::fitness::fitness;
use super::initialization::init_data;
use super::neighbors::neighbors;

type Point = Vec<f64>;

pub fn simulated_annealing<F>(start_point: &Point, end_point: &Point, test_fn: &F, num_controls: u8)
-> Vec<f64>
where F: Fn(f64) -> f64 {
	let start_temp = 1.0;
	let end_temp = 0.001;
	let alpha = 0.99;

	let mut temperature = start_temp;
	let mut data = init_data(start_point, end_point, num_controls);

	let samples = 1000;
	let initial_fitness = fitness(&data, start_point, end_point, test_fn, samples);

	while temperature > end_temp {
		data = step_data(&data, start_point, end_point, test_fn, samples, temperature, initial_fitness);
		temperature *= alpha;
	}

	data
}

fn step_data<F>(data: &Vec<f64>, start_point: &Point, end_point: &Point, test_fn: &F, samples: u32, temperature: f32, initial_fitness: f32) -> Vec<f64>
where F: Fn(f64) -> f64 {
	let domain = (end_point[0] - start_point[0]) as f32;
	let current_fitness = fitness(&data, start_point, end_point, test_fn, samples);
	let delta = delta(domain, current_fitness, initial_fitness, temperature);

	let neighbors = neighbors(data, delta as f64);

	let mut probabilities = Vec::new();

	for neighbor in neighbors.clone() {
		let step_fitness = fitness(&neighbor, start_point, end_point, test_fn, samples);
		let probability = probability(current_fitness, step_fitness, temperature);

		probabilities.push(probability);
	}

	let choice = pick_from_weight_list(&probabilities);

	let choice_fitness = fitness(&neighbors[choice], start_point, end_point, test_fn, samples);
	println!("Fitness: {choice_fitness}, delta: {delta}");

	neighbors[choice].clone()
}

fn probability(current: f32, step: f32, temperature: f32) -> f32 {
	temperature.powf((step - current) / 10.0)
}

fn delta(domain: f32, fitness: f32, initial_fitness: f32, temperature: f32) -> f32 {
	(domain / 10.0) * (1.0 - (1.0 - (5.0 / initial_fitness)).powf(fitness)) * temperature.powf(1.0 / 5.0)
}
