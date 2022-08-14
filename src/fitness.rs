use genevo::genetic::FitnessFunction;

use crate::{genotype::Geno, model::raytrace, paramset::ParamSet};

#[derive(Clone, Copy, Debug)]
struct AgileFitness;

impl FitnessFunction<Geno, u64> for AgileFitness {
	fn fitness_of(&self, geno: &Geno) -> u64 {
		raytrace(ParamSet::from(*geno)).summarise()
	}

	fn average(&self, a: &[u64]) -> u64 {
		let sum: u64 = a.iter().sum();
		let len: u64 = a.len().try_into().unwrap_or(u64::MAX);
		sum / len
	}

	fn highest_possible_fitness(&self) -> u64 {
		u64::MAX
	}

	fn lowest_possible_fitness(&self) -> u64 {
		0
	}
}

impl FitnessFunction<ParamSet, u64> for AgileFitness {
	fn fitness_of(&self, params: &ParamSet) -> u64 {
		raytrace(*params).summarise()
	}

	fn average(&self, a: &[u64]) -> u64 {
		let sum: u64 = a.iter().sum();
		let len: u64 = a.len().try_into().unwrap_or(u64::MAX);
		sum / len
	}

	fn highest_possible_fitness(&self) -> u64 {
		u64::MAX
	}

	fn lowest_possible_fitness(&self) -> u64 {
		0
	}
}
