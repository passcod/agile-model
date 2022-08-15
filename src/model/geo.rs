use std::fmt::Display;

use super::{
	refract::{snells, Snells},
	units::{Microns, Radians},
};

#[derive(Clone, Copy, Default, Debug)]
pub struct Pos {
	/// Horizontal position from leftmost
	pub x: Microns,

	/// Vertical height
	pub y: Microns,

	/// Current RI
	pub ri: u8,

	/// Current direction (from normal)
	pub dir: Radians,

	/// Vertical direction
	pub going_down: bool,
}

impl Pos {
	/// Recomputes directions from the RI change at a boundary.
	///
	/// Also does total internal reflection as needed.
	pub fn refract_into(&mut self, new_ri: u8) {
		if self.dir.signum() == 0.0 || new_ri == self.ri {
			// no refraction happens
		} else {
			match snells(self.ri, new_ri, self.dir) {
				(Snells::Refracted, angle) => {
					self.dir = angle;
				}
				(Snells::Reflected, angle) => {
					self.dir = angle;
					self.going_down = !self.going_down;
				}
				(Snells::Critical, angle) => {
					self.dir = angle;
					self.going_down = true; // not really, but ah well
				}
			}
		}

		self.ri = new_ri;
	}

	/// Takes vertical distances to boundaries above and below,
	/// outputs travel distance.
	///
	/// Also does reflection as needed.
	pub fn travel_to_next_boundary(&mut self, up: Microns, down: Microns) -> Microns {
		todo!("travel")
	}
}

#[derive(Clone, Copy, Debug)]
struct Line {
	pub slope: f64,
	pub y_intercept: i64,
}

impl Display for Line {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "y = {}x {}", self.slope, self.y_intercept)
	}
}
