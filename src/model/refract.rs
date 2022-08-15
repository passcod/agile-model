use std::f64::consts::PI;

use ang::Angle;

use super::units::model_ri_to_real_ri;

/// Converts an absolute direction to an incidence to the normal.
///
/// The normals in our case are always vertical. A positive incidence is in the
/// quadrants 3 and 4 of the unit circle ("going left").
pub const fn normalise_incidence(direction: Angle) -> Angle {
	todo!()
}

/// Converts an incidence against the normal to an absolute direction.
pub const fn denormalise_incidence(incidence: Angle) -> Angle {
	todo!()
}

/// Calculates refraction between two mediums for a ray going a certain direction.
///
/// Returns the output direction.
pub fn snells(old_ri: u8, new_ri: u8, direction: Angle) -> Angle {
	let old_ri = model_ri_to_real_ri(old_ri);
	let new_ri = model_ri_to_real_ri(new_ri);
	let incidence = normalise_incidence(direction);

	let new_sin = (old_ri / new_ri) * incidence.sin();
	let abs_sin = new_sin.abs();
	if abs_sin < 1.0 {
		// refraction
		denormalise_incidence(ang::asin(new_sin).unwrap())
	} else if abs_sin > 1.0 {
		// total internal reflection
		denormalise_incidence(-incidence)
	} else {
		// critical angle
		denormalise_incidence(incidence.signum() * (PI / 2.0))
	}
}
