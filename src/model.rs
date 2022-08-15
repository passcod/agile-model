use std::f64::consts::PI;

use crate::paramset::ParamSet;

use geo::Pos;
use units::{mm_tenths_to_microns, Microns, Radians, RI_AIR};

pub mod geo;
pub mod refract;
pub mod units;

#[derive(Clone, Copy, Debug)]
pub struct Performance {
	/// Proportion of rays that exit at the bottom.
	///
	/// Calculated as {bottom exit rays} * u32::MAX / {total rays}.
	///
	/// Higher is better.
	pub exit_ratio: u32,

	/// Average of exit angles (to the normal) for rays that exit at the bottom.
	///
	/// In 10000th radians.
	///
	/// Lower is better.
	pub exit_angle: u32,

	/// Total distance light travels inside the lens.
	///
	/// In micrometres.
	///
	/// Lower is better.
	pub light_travel: u32,
}

impl Performance {
	pub fn summarise(self) -> u64 {
		let one = self.exit_ratio as u64;
		let two = u32::MAX.saturating_sub(self.exit_angle) as u64;
		let three = u32::MAX.saturating_sub(self.light_travel) as u64;

		one * 10 + two * 5 + three
	}
}

const ENTRY_INTERVAL: Microns = 1000;
const ANGLE_INTERVAL: Radians = PI / 180.0;
const ANGLE_MAX: Radians = PI / 2.0;
const ANGLE_MIN: Radians = -ANGLE_MAX;

pub fn raytrace(params: ParamSet) -> Performance {
	let mut traces = Vec::with_capacity(18960);

	let mut entry: Microns = 0;
	while entry <= ParamSet::WIDTH_TOP {
		entry += ENTRY_INTERVAL;
		let mut angle: Radians = ANGLE_MIN;
		while angle <= ANGLE_MAX {
			angle += ANGLE_INTERVAL;
			traces.push(trace_one(params, entry, angle));
		}
	}

	let total_rays = traces.len();
	let (bottom_angles, bottom_travel): (Vec<Radians>, Vec<Microns>) = traces
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
	let average_angle: Radians =
		bottom_angles.iter().map(|a| a.abs()).sum::<Radians>() / (total_bottomed as Radians);

	Performance {
		exit_ratio: ((total_bottomed * (u32::MAX as usize)) / total_rays) as _,
		exit_angle: (average_angle * 10000.0) as _,
		light_travel: total_travel as _,
	}
}

#[derive(Clone, Copy, Debug)]
enum Traced {
	TopExit,
	BottomExit { angle: Radians, travel: Microns },
}

fn trace_one(params: ParamSet, entry_point: Microns, entry_angle: Radians) -> Traced {
	let part_um = mm_tenths_to_microns(params.partitions_thickness);
	let layer_um = mm_tenths_to_microns(params.layers_thickness).saturating_add(3_000);

	let layers: Vec<u8> = params
		.layers
		.iter()
		.filter_map(|l| l.map(|n| n.get()))
		.collect();

	let lens_height = (layers.len() as u64) * (part_um + layer_um);

	// all this kinda considers that the bottom partition doesn't exist :/
	// ...FIXME maybe?

	let mut travel: Microns = 0;
	let mut ray = Pos {
		x: entry_point,
		y: lens_height,
		ri: RI_AIR,
		dir: entry_angle,
		going_down: true,
	};

	let mut layer: usize = 0;
	let mut in_layer = false;
	let mut boundary_above = lens_height;
	let mut boundary_below = lens_height.saturating_sub(part_um);

	// top partition entry
	ray.refract_into(ParamSet::MAXIMUM_RI);

	loop {
		let so_far = ray.travel_to_next_boundary(boundary_above, boundary_below);
		travel += so_far;

		if ray.y > lens_height || (ray.y == lens_height && !ray.going_down) {
			break Traced::TopExit;
		}

		if in_layer {
			// refract into partition from layer
			ray.refract_into(ParamSet::MAXIMUM_RI);
			in_layer = false;

			if ray.going_down {
				boundary_above = ray.y;
				boundary_below = ray.y - part_um;
			} else {
				boundary_above = ray.y + part_um;
				boundary_below = ray.y;
			}
		} else {
			// refract into layer from partition
			in_layer = true;

			if ray.going_down {
				layer += 1;
				boundary_above = ray.y;
				boundary_below = ray.y - layer_um;
			} else if layer == 0 {
				break Traced::TopExit;
			} else {
				layer -= 1;
				boundary_above = ray.y + layer_um;
				boundary_below = ray.y;
			};

			if let Some(ri) = layers.get(layer) {
				ray.refract_into(*ri);
			} else {
				break Traced::BottomExit {
					angle: ray.dir,
					travel,
				};
			}
		}

		if ray.y == 0 {
			break Traced::BottomExit {
				angle: ray.dir,
				travel,
			};
		}
	}
}
