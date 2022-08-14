use genevo::genetic::FitnessFunction;

use crate::{model::raytrace, paramset::ParamSet};

#[derive(Clone, Copy, Debug)]
pub struct AgileFitness;

impl FitnessFunction<ParamSet, u64> for AgileFitness {
	fn fitness_of(&self, params: &ParamSet) -> u64 {
		raytrace(*params).summarise()
	}

	fn average(&self, a: &[u64]) -> u64 {
		let sum = a
			.iter()
			.copied()
			.reduce(|sum, e| sum.saturating_add(e))
			.unwrap_or_default();
		let len = a.len().try_into().unwrap_or(u64::MAX);
		sum / len
	}

	fn highest_possible_fitness(&self) -> u64 {
		u64::MAX
	}

	fn lowest_possible_fitness(&self) -> u64 {
		0
	}
}
