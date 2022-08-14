use genevo::{
	genetic::{Children, Parents},
	operator::{CrossoverOp, GeneticOperator},
	random::Rng,
};

use crate::{array::ParamArray, paramset::ParamSet};

use super::old_value;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct AgileCrossBreeder;

impl GeneticOperator for AgileCrossBreeder {
	fn name() -> String {
		"Uniform-Cross-Breeder".to_string()
	}
}

impl CrossoverOp<ParamSet> for AgileCrossBreeder {
	fn crossover<R>(&self, parents: Parents<ParamSet>, rng: &mut R) -> Children<ParamSet>
	where
		R: Rng + Sized,
	{
		let num_parents = parents.len();

		// breed one child for each partner in parents
		let mut offspring: Vec<ParamArray> = Vec::with_capacity(num_parents);
		while num_parents > offspring.len() {
			let mut genome = ParamArray::default();
			// for each value in the genotype
			for locus in 0..ParamSet::LAYERS {
				// pick the value of a randomly chosen parent
				let random = rng.gen_range(0..num_parents);
				let value = old_value(parents[random], locus);
				genome.set(locus, value);
			}
			offspring.push(genome);
		}

		offspring.into_iter().map(ParamSet::from).collect()
	}
}
