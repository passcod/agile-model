use std::num::NonZeroU8;

use crate::paramset::{ParamSet, normalise_partition_thickness};

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
			partitions_thickness: field[1],
			layers: layers.map(NonZeroU8::new),
		}
	}
}

impl From<ParamSet> for ParamArray {
	fn from(params: ParamSet) -> Self {
		let mut field = [0; ParamSet::LAYERS + 2];
		field[0] = params.layers_thickness;
		field[1] = normalise_partition_thickness(params.partitions_thickness);
		field[2..].copy_from_slice(&params.layers.map(|n| n.map_or(0, |n| n.get())));
		Self(field)
	}
}
