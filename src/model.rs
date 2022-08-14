use crate::paramset::ParamSet;

#[derive(Clone, Copy, Debug)]
pub struct Performance {
	/// Proportion of rays that exit at the bottom.
	///
	/// Calculated as {bottom exit rays} * u16::MAX / {total rays}.
	///
	/// Higher is better.
	pub exit_ratio: u16,

	/// Average of exit angles (to the normal) for rays that exit at the bottom.
	///
	/// In 40000th of radians.
	///
	/// Lower is better.
	pub exit_angle: u16,

	/// Total distance light travels inside the lens.
	///
	/// In micrometres.
	///
	/// Lower is better.
	pub light_travel: u32,
}

impl Performance {
	pub fn summarise(self) -> u64 {
		let one = self.exit_ratio.to_be_bytes();
		let two = u16::MAX.saturating_sub(self.exit_angle).to_be_bytes();
		let three = u32::MAX.saturating_sub(self.light_travel).to_be_bytes();
		u64::from_be_bytes([
			one[0], one[1], two[0], two[1], three[0], three[1], three[2], three[3],
		])
	}
}

pub fn raytrace(params: ParamSet) -> Performance {
	todo!()
}
