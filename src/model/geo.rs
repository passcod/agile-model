use std::fmt::Display;

use super::units::Microns;

#[derive(Clone, Copy, Default, Debug)]
pub struct Point {
	/// Horizontal position from leftmost
	pub x: Microns,

	/// Vertical height
	pub y: Microns,
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
	pub slope: f64,
	pub y_intercept: i64,
}

impl Line {
	pub fn new(slope: f64, y_intercept: i64) -> Self {
		Self { slope, y_intercept }
	}

	pub fn intersection(self, other: Self) -> Option<Point> {
		todo!()
	}
}

impl Display for Line {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match (self.slope == 0.0, self.y_intercept == 0) {
			(true, true) => write!(f, "y = 0"),
			(true, false) => write!(
				f,
				"y = {}{}",
				if self.y_intercept > 0 { "" } else { "-" },
				self.y_intercept
			),
			(false, true) => write!(f, "y = {}x", self.slope),
			(false, false) => write!(
				f,
				"y = {}x {} {}",
				self.slope,
				if self.y_intercept > 0 { "+" } else { "-" },
				self.y_intercept
			),
		}
	}
}
