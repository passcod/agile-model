use std::num::NonZeroU8;

use crate::paramset::{normalise_partition_thickness, ParamSet};

pub mod breeder;
pub mod crossover;
pub mod random;

fn prep<R>(mutation_rate: f64, rng: &mut R) -> (usize, usize)
where
	R: genevo::random::Rng + Sized,
{
	let genome_length = 3 + ParamSet::LAYERS;
	let num_mutations = ((genome_length as f64 * mutation_rate) + rng.gen::<f64>()).floor() as _;
	(genome_length, num_mutations)
}

fn old_value(genome: ParamSet, index: usize) -> u8 {
	match index {
		0 => genome.len() as u8,
		1 => genome.layers_thickness,
		2 => genome.partitions_thickness,
		n => genome.layers[n - 3].map_or(0, |ri| ri.get()),
	}
}

fn apply_value(genome: &mut ParamSet, index: usize, new: u8) {
	match index {
		0 => {
			let current_len = genome.len();
			let new_len = new as usize;
			if current_len < new_len {
				genome.layers[current_len..new_len].fill(NonZeroU8::new(ParamSet::MINIMUM_RI));
			} else if current_len > new_len {
				genome.layers[(new_len - 1)..].fill(None);
			}
		}
		1 => {
			genome.layers_thickness = new;
		}
		2 => {
			genome.partitions_thickness = normalise_partition_thickness(new);
		}
		n => {
			genome.layers[n - 3] = NonZeroU8::new(if new == 0 {
				0
			} else {
				new.max(ParamSet::MINIMUM_RI)
			});
		}
	}
}
