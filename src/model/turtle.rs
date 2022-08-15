use super::{
	geo::Point,
	refract::{snells, Snells},
	units::{Microns, Radians},
};

#[derive(Clone, Copy, Default, Debug)]
pub struct Turtle {
	/// Position
	pub pos: Point,

	/// Current RI
	pub ri: u8,

	/// Current direction (from normal)
	pub dir: Radians,

	/// Vertical direction
	pub going_down: bool,
}

impl Turtle {
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
