use crate::paramset::ParamSet;

use ang::Angle;
use geo::Point;
use turtle::Turtle;
use units::{mm_tenths_to_microns, Microns, RI_AIR};

use self::refract::normalise_incidence;

pub mod geo;
pub mod refract;
pub mod turtle;
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
	/// In 10000th Angle.
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

const ENTRY_INTERVAL: Microns = 5_000;
const ANGLE_INTERVAL: Angle = Angle::Degrees(5.0);
const ANGLE_MIN: Angle = Angle::Degrees(-90.0);
const ANGLE_MAX: Angle = Angle::Degrees(90.0);

pub fn raytrace(params: ParamSet) -> Performance {
	let mut traces = Vec::with_capacity(18960);

	let mut entry: Microns = 0;
	while entry <= ParamSet::WIDTH_TOP {
		entry += ENTRY_INTERVAL;
		let mut angle: Angle = ANGLE_MIN;
		while angle <= ANGLE_MAX {
			angle += ANGLE_INTERVAL;
			traces.push(trace_one(params, entry, angle));
		}
	}

	let total_rays = traces.len();
	let (bottom_angles, bottom_travel): (Vec<Angle>, Vec<Microns>) = traces
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
	let average_angle: Angle = Angle::Radians(
		bottom_angles
			.iter()
			.map(|a| a.abs().in_radians())
			.sum::<f64>()
			/ (total_bottomed as f64),
	);

	Performance {
		exit_ratio: ((total_bottomed * (u32::MAX as usize)) / total_rays) as _,
		exit_angle: (average_angle.in_radians() * 10000.0) as _,
		light_travel: total_travel as _,
	}
}

#[derive(Clone, Copy, Debug)]
enum Traced {
	TopExit,
	BottomExit { angle: Angle, travel: Microns },
}

fn trace_one(params: ParamSet, entry_point: Microns, entry_angle: Angle) -> Traced {
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
	let mut ray = Turtle {
		pos: Point {
			x: entry_point,
			y: lens_height,
		},
		ri: RI_AIR,
		dir: entry_angle,
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

		if ray.pos.y > lens_height || (ray.pos.y == lens_height && !ray.is_going_down()) {
			break Traced::TopExit;
		}

		if in_layer {
			// refract into partition from layer
			ray.refract_into(ParamSet::MAXIMUM_RI);
			in_layer = false;

			if ray.is_horizontal() {
				// no change
			} else if ray.is_going_down() {
				boundary_above = ray.pos.y;
				boundary_below = ray.pos.y - part_um;
			} else {
				boundary_above = ray.pos.y + part_um;
				boundary_below = ray.pos.y;
			}
		} else {
			// refract into layer from partition
			in_layer = true;

			if ray.is_horizontal() {
				// no change
			} else if ray.is_going_down() {
				layer += 1;
				boundary_above = ray.pos.y;
				boundary_below = ray.pos.y - layer_um;
			} else if layer == 0 {
				break Traced::TopExit;
			} else {
				layer -= 1;
				boundary_above = ray.pos.y + layer_um;
				boundary_below = ray.pos.y;
			};

			if let Some(ri) = layers.get(layer) {
				ray.refract_into(*ri);
			} else {
				break Traced::BottomExit {
					angle: normalise_incidence(ray.dir),
					travel,
				};
			}
		}

		if ray.pos.y == 0 {
			break Traced::BottomExit {
				angle: normalise_incidence(ray.dir),
				travel,
			};
		}
	}
}
