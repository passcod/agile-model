use std::f64::consts::PI;

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
	/// In milliradians.
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

type Microns = u64; // forwards from leftmost
type Millirads = i64; // negative is backwards

const ENTRY_INTERVAL: Microns = 1000;
const ANGLE_INTERVAL: Millirads = ((PI / 180.0) * 1000.0) as Millirads;
const ANGLE_MAX: Millirads = (PI * 1000.0) as Millirads - ANGLE_INTERVAL;

fn top_width(_: ParamSet) -> Microns {
	104 * 1000
}

pub fn raytrace(params: ParamSet) -> Performance {
	let mut traces = Vec::with_capacity(18960);

	let top_width = top_width(params);
	let mut entry: Microns = 0;
	while entry <= top_width {
		entry += ENTRY_INTERVAL;
		let mut angle: Millirads = 0;
		while angle <= ANGLE_MAX {
			angle += ANGLE_INTERVAL;
			traces.push(trace_one(params, entry, angle));
		}
	}

	let total_rays = traces.len();
	let (bottom_angles, bottom_travel): (Vec<Millirads>, Vec<Microns>) = traces
		.iter()
		.filter_map(|t| {
			if let Traced::BottomExit { angle, travel } = t {
				Some((angle, travel))
			} else {
				None
			}
		})
		.unzip();
	let total_bottomed = bottom_angles.len();
	let total_travel: Microns = bottom_travel.into_iter().sum();
	let average_angle: Millirads = bottom_angles.iter().sum::<Millirads>()
		/ Millirads::try_from(total_bottomed).unwrap_or(Millirads::MAX);

	Performance {
		exit_ratio: ((total_bottomed * (u16::MAX as usize)) / total_rays) as _,
		exit_angle: average_angle as _,
		light_travel: total_travel as _,
	}
}

#[derive(Clone, Copy, Debug)]
enum Traced {
	TopExit,
	BottomExit { angle: Millirads, travel: Microns },
}

fn trace_one(params: ParamSet, entry_point: Microns, entry_angle: Millirads) -> Traced {
	let mut travel: Microns = 0;
	let mut angle = entry_angle;

	todo!()
}
