use genevo::{prelude::GenomeBuilder, random::Rng};

use crate::paramset::ParamSet;

pub struct RandomBuilder;

impl GenomeBuilder<ParamSet> for RandomBuilder {
	fn build_genome<R>(&self, _: usize, rng: &mut R) -> ParamSet
	where
		R: Rng + Sized,
	{
		ParamSet::nth(rng.gen_range(0..ParamSet::MAX_POSSIBILITIES))
	}
}
