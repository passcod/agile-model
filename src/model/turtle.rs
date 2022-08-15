use std::f64::consts::PI;

use ang::Angle;

use super::{geo::Point, refract::snells, units::Microns};

pub const QUARTER: Angle = Angle::Radians(PI / 2.0);
pub const HALF: Angle = Angle::Radians(PI);
pub const THREE_QUARTERS: Angle = Angle::Radians(PI + PI / 2.0);
pub const FULL: Angle = Angle::Radians(2.0 * PI);

#[derive(Clone, Copy, Default, Debug)]
pub struct Turtle {
	/// Position
	pub pos: Point,

	/// Current RI
	pub ri: u8,

	/// Current direction
	///
	/// 0 is up.
	pub dir: Angle,
}

impl Turtle {
	pub fn is_going_up(self) -> bool {
		self.dir < QUARTER || self.dir > THREE_QUARTERS
	}

	pub fn is_going_right(self) -> bool {
		self.dir < HALF
	}

	pub fn is_going_down(self) -> bool {
		self.dir > QUARTER && self.dir < THREE_QUARTERS
	}

	pub fn is_going_left(self) -> bool {
		self.dir > HALF
	}

	pub fn is_horizontal(self) -> bool {
		!self.is_going_up() && !self.is_going_down()
	}

	pub fn is_vertical(self) -> bool {
		!self.is_going_left() && !self.is_going_right()
	}

	pub fn turn(&mut self, by: Angle) {
		self.dir = (self.dir + by).normalized();
	}

	/// Recomputes directions from the RI change at a boundary.
	///
	/// Also does total internal reflection as needed.
	pub fn refract_into(&mut self, new_ri: u8) {
		if self.is_vertical() || self.is_horizontal() || new_ri == self.ri {
			// no refraction happens
		} else {
			self.dir = snells(self.ri, new_ri, self.dir);
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
