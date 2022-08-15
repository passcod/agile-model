use std::num::NonZeroU8;

use crate::paramset::{normalise_partition_thickness, ParamSet, PARTITION_THICKNESSES};

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

const PART_THICKNESS_RANGE: u8 =
	PARTITION_THICKNESSES[PARTITION_THICKNESSES.len() - 1] - PARTITION_THICKNESSES[0];

fn old_value(genome: ParamSet, index: usize) -> u8 {
	match index {
		0 => genome.len() as u8,
		1 => genome.layers_thickness,
		2 => genome.partitions_thickness * (u8::MAX / PART_THICKNESS_RANGE),
		n => genome.layers[n - 3].map_or(0, |ri| ri.get()),
	}
}

fn apply_value(genome: &mut ParamSet, index: usize, new: u8) {
	match index {
		0 => {
			let current_len = genome.len();
			let new_len = ParamSet::LAYERS.min((new as usize) % (ParamSet::LAYERS + 1));
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
			let new_part = (new * PART_THICKNESS_RANGE) / u8::MAX;
			genome.partitions_thickness = normalise_partition_thickness(new_part);
		}
		n => {
			if let Some(ri) = &mut genome.layers[n - 3] {
				*ri = unsafe { NonZeroU8::new_unchecked(new.max(ParamSet::MINIMUM_RI)) };
			}
		}
	}
}
