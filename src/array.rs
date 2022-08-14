use std::num::NonZeroU8;

use crate::paramset::{normalise_partition_thickness, ParamSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ParamArray([u8; ParamSet::LAYERS + 2]);

impl Default for ParamArray {
	fn default() -> Self {
		ParamSet::default().into()
	}
}

impl ParamArray {
	/// Panics if index is out of bounds
	pub fn set(&mut self, index: usize, value: u8) {
		self.0[index] = value;
	}
}

// Could probably transmute instead, with a fixed repr?

impl From<ParamArray> for ParamSet {
	fn from(geno: ParamArray) -> Self {
		let field = geno.0;
		let mut layers = [0; ParamSet::LAYERS];
		layers.copy_from_slice(&field[2..]);
		Self {
			layers_thickness: field[0],
			partitions_thickness: normalise_partition_thickness(field[1]),
			layers: layers.map(|ri| {
				if ri == 0 {
					None
				} else {
					Some(unsafe { NonZeroU8::new_unchecked(ri.max(ParamSet::MINIMUM_RI)) })
				}
			}),
		}
	}
}

impl From<ParamSet> for ParamArray {
	fn from(params: ParamSet) -> Self {
		let mut field = [0; ParamSet::LAYERS + 2];
		field[0] = params.layers_thickness;
		field[1] = normalise_partition_thickness(params.partitions_thickness);
		field[2..].copy_from_slice(
			&params
				.layers
				.map(|n| n.map_or(0, |n| n.get().max(ParamSet::MINIMUM_RI))),
		);
		Self(field)
	}
}
