use genevo::{
	genetic::Genotype,
	operator::prelude::{BreederGenomeMutation, BreederValueMutation, RandomValueMutation},
	random::{random_index, SliceRandom},
};

use crate::paramset::ParamSet;

use super::{apply_value, old_value, prep};

impl BreederGenomeMutation for ParamSet {
	type Dna = u8;

	fn mutate_genome<R>(
		genome: Self,
		mutation_rate: f64,
		range: &<Self as Genotype>::Dna,
		precision: u8,
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
			let sign = *[-1, 1].choose(rng).unwrap();
			let adjustment = if *[true, false].choose(rng).unwrap() {
				1. / (1i64 << precision) as f64
			} else {
				1.
			};

			let old = old_value(mutated, index);
			let value_mut = BreederValueMutation::breeder_mutated(old, range, adjustment, sign);

			apply_value(
				&mut mutated,
				index,
				if value_mut < *min_value {
					RandomValueMutation::random_mutated(value_mut, min_value, max_value, rng)
				} else if value_mut > *max_value {
					*max_value
				} else {
					value_mut
				},
			);
		}
		mutated
	}
}
