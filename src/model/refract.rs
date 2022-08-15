use std::f64::consts::PI;

use super::units::{model_ri_to_real_ri, Radians};

pub fn snells(old_ri: u8, new_ri: u8, angle: Radians) -> (Snells, Radians) {
	let old_ri = model_ri_to_real_ri(old_ri);
	let new_ri = model_ri_to_real_ri(new_ri);

	let new_sin = (old_ri / new_ri) * angle.sin();
	let abs_sin = new_sin.abs();
	if abs_sin < 1.0 {
		(Snells::Refracted, new_sin.asin())
	} else if abs_sin > 1.0 {
		(Snells::Reflected, -angle)
	} else {
		(Snells::Critical, angle.signum() * (PI / 2.0))
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snells {
	Refracted,
	Critical,
	Reflected,
}
