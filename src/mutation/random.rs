use genevo::{
	genetic::Genotype,
	mutation::value::{RandomGenomeMutation, RandomValueMutation},
	random::random_index,
};

use crate::paramset::ParamSet;

use super::{apply_value, old_value, prep};

impl RandomGenomeMutation for ParamSet {
	type Dna = u8;

	fn mutate_genome<R>(
		genome: Self,
		mutation_rate: f64,
		min_value: &<Self as Genotype>::Dna,
		max_value: &<Self as Genotype>::Dna,
		rng: &mut R,
	) -> Self
	where
		R: genevo::random::Rng + Sized,
	{
		let (genome_length, num_mutations) = prep(mutation_rate, rng);

		let mut mutated = genome;
		for _ in 0..num_mutations {
			let index = random_index(rng, genome_length);

			let old = old_value(mutated, index);
			let new = RandomValueMutation::random_mutated(old, min_value, max_value, rng);
			apply_value(&mut mutated, index, new);
		}

		mutated
	}
}
