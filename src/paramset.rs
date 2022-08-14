use std::{fmt::Display, num::NonZeroU8};

use genevo::genetic::Genotype;

/// Convert from model RI to real RI
pub fn model_ri_to_real_ri(model_ri: u8) -> f64 {
	f64::from(model_ri) * 0.01 + 0.99
}

// Set of usable paritition thicknesses.
///
/// These are in tenths of millimetres, like in [`ParamSet`].
pub const PARTITION_THICKNESSES: [u8; 9] = [2, 4, 6, 8, 10, 12, 15, 20, 30];

/// From an arbitrary thickness, find the closest usable one.
pub fn normalise_partition_thickness(thicc: u8) -> u8 {
	if thicc <= PARTITION_THICKNESSES[0] {
		PARTITION_THICKNESSES[0]
	} else if thicc >= PARTITION_THICKNESSES[8] {
		PARTITION_THICKNESSES[8]
	} else {
		match PARTITION_THICKNESSES.binary_search(&thicc) {
			Ok(n) => PARTITION_THICKNESSES[n],
			Err(closest_up) => {
				let dist_up = PARTITION_THICKNESSES[closest_up].saturating_sub(thicc);
				let dist_down = thicc.saturating_sub(PARTITION_THICKNESSES[closest_up - 1]);
				if dist_up > dist_down {
					PARTITION_THICKNESSES[closest_up]
				} else {
					PARTITION_THICKNESSES[closest_up - 1]
				}
			}
		}
	}
}

// Parameter set for an AGILE.
///
/// This is optimised for struct size, instead of ease of use: it is 12 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ParamSet {
	// Thickness of each layer in tenths of mm above 3.0mm.
	pub layers_thickness: u8,

	// Thickness of each partition in tenths of mm
	pub partitions_thickness: u8,

	// Refractive indices of the layers in hundredths above 0.99
	///
	/// - 0 = None
	/// - 1 = 1.00 (air/vacuum)
	/// - 34 = 1.33 (water)
	/// - 51 = 1.50 (acrylic)
	///
	/// There are 10 possible layers; those that are Some are the ones defined
	/// for this parameter set.
	pub layers: [Option<NonZeroU8>; Self::LAYERS],
}

impl Genotype for ParamSet {
	type Dna = u8;
}

impl Default for ParamSet {
	fn default() -> Self {
		Self::nth(0)
	}
}

impl Display for ParamSet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let layers_mm = f32::from(self.layers_thickness) * 0.1 + 3.0;
		let parts_mm = f32::from(self.partitions_thickness) * 0.1;
		write!(f, "layer:{layers_mm:.02}mm part:{parts_mm:.02}mm  | ")?;
		for ri in &self.layers {
			if let Some(ri) = ri {
				let ri = model_ri_to_real_ri(ri.get());
				write!(f, "{ri:.02} ")?;
			} else {
				break;
			}
		}
		write!(f, "|")
	}
}

impl ParamSet {
	pub const POSSIBLE_LAYERS: u64 = u8::MAX as _;
	pub const POSSIBLE_PARTS: u64 = PARTITION_THICKNESSES.len() as _;

	pub const LAYERS: usize = 10;
	pub const MINIMUM_RI: u8 = 34; // 0.99 (None) + 0.34 = 1.33 (water)
	pub const MAXIMUM_RI: u8 = 51; // 0.99 (None) + 0.51 = 1.50 (acrylic)

	// +1 for the None possibility
	pub const POSSIBLE_RIS: u64 = (1 + Self::MAXIMUM_RI - Self::MINIMUM_RI) as _;

	// -1 because because it overflows the possibility space!
	pub const MAX_POSSIBILITIES: u64 =
		Self::POSSIBLE_LAYERS * Self::POSSIBLE_PARTS * Self::POSSIBLE_RIS.pow(Self::LAYERS as _)
			- 1;

	/// Generate the Nth parameter set.
	///
	/// Sequence loops breadth first through:
	/// - thickness of layers (in 0.1mm increments, 256 steps)
	/// - thickness of partitions (out of [`PARTITION_THICKNESSES`], 9 steps)
	/// - RI of each layer (in 0.01 increments, starting at None, 18 steps each)
	pub fn nth(mut n: u64) -> Self {
		let layer_n = u8::try_from(n % Self::POSSIBLE_LAYERS).unwrap();
		n /= Self::POSSIBLE_LAYERS;
		let part_n = usize::try_from(n % Self::POSSIBLE_PARTS).unwrap();
		n /= Self::POSSIBLE_PARTS;

		let mut ris = [None::<NonZeroU8>; Self::LAYERS];
		for ri in &mut ris {
			*ri = NonZeroU8::new(
				ri.map_or(Self::MINIMUM_RI, |u| u.get())
					+ u8::try_from(n % Self::POSSIBLE_RIS).unwrap(),
			);

			if n == 0 {
				break;
			}

			n /= Self::POSSIBLE_RIS;
		}

		Self {
			layers_thickness: layer_n,
			partitions_thickness: PARTITION_THICKNESSES[part_n],
			layers: ris,
		}
	}

	/// Length of the layers "array".
	pub fn len(self) -> usize {
		self.layers
			.iter()
			.enumerate()
			.filter(|(_, layer)| layer.is_none())
			.next()
			.map_or(0, |(n, _)| n)
	}
}
