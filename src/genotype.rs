use std::num::NonZeroU8;

use genevo::genetic::Genotype;

use crate::paramset::ParamSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Geno([u8; ParamSet::LAYERS + 2]);

impl Genotype for Geno {
	type Dna = u8;
}

impl Genotype for ParamSet {
	type Dna = u8;
}

// Could probably transmute instead, with a fixed repr?

impl From<Geno> for ParamSet {
	fn from(geno: Geno) -> Self {
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

impl From<ParamSet> for Geno {
	fn from(params: ParamSet) -> Self {
		let mut field = [0; ParamSet::LAYERS + 2];
		field[0] = params.layers_thickness;
		field[1] = params.partitions_thickness;
		field[2..].copy_from_slice(&params.layers.map(|n| n.map_or(0, |n| n.get())));
		Self(field)
	}
}
