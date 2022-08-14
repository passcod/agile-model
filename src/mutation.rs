use std::num::NonZeroU8;

use crate::paramset::ParamSet;

pub mod breeder;
pub mod crossover;
pub mod random;

fn prep<R>(genome: ParamSet, mutation_rate: f64, rng: &mut R) -> (usize, usize)
where
	R: genevo::random::Rng + Sized,
{
	let genome_length = 3 + genome.len();
	let num_mutations = ((genome_length as f64 * mutation_rate) + rng.gen::<f64>()).floor() as _;
	(genome_length, num_mutations)
}

fn old_value(genome: ParamSet, index: usize) -> u8 {
	match index {
		0 => 0, // push one layer up or down?
		1 => genome.layers_thickness,
		2 => genome.partitions_thickness,
		n => genome.layers[n - 3].map_or(0, |ri| ri.get()),
	}
}

fn apply_value(genome: &mut ParamSet, index: usize, new: u8) {
	match index {
		0 => {
			if genome.len() < ParamSet::LAYERS && new > (u8::MAX / 2) {
				genome.layers[genome.len() - 1] = NonZeroU8::new(new);
			} else if genome.len() > 1 && new < (u8::MAX / 2) {
				genome.layers[genome.len() - 1] = None;
			}
		}
		1 => {
			genome.layers_thickness = new;
		}
		2 => {
			genome.partitions_thickness = new;
		}
		n => {
			genome.layers[n - 3] = NonZeroU8::new(new);
		}
	}
}
