use std::f64::consts::PI;

use super::units::{model_ri_to_real_ri, Radians};

/// Converts an absolute direction to an incidence to the normal.
///
/// The normals in our case are always vertical.
pub const fn normalise_incidence(direction: Radians) -> Radians {
	todo!()
}

/// Converts an incidence against the normal to an absolute direction.
pub const fn denormalise_incidence(incidence: Radians) -> Radians {
	todo!()
}

/// Calculates refraction between two mediums for a ray going a certain direction.
///
/// Returns the output direction.
pub fn snells(old_ri: u8, new_ri: u8, direction: Radians) -> Radians {
	let old_ri = model_ri_to_real_ri(old_ri);
	let new_ri = model_ri_to_real_ri(new_ri);
	let incidence = normalise_incidence(direction);

	let new_sin = (old_ri / new_ri) * incidence.sin();
	let abs_sin = new_sin.abs();
	if abs_sin < 1.0 {
		// refraction
		denormalise_incidence(new_sin.asin())
	} else if abs_sin > 1.0 {
		// total internal reflection
		denormalise_incidence(-incidence)
	} else {
		// critical angle
		denormalise_incidence(incidence.signum() * (PI / 2.0))
	}
}
