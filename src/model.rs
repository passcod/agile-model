use std::{cmp::Ordering, f64::consts::PI};

use crate::paramset::{model_ri_to_real_ri, ParamSet};

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

type Microns = u64; // forwards from leftmost
type Radians = f64; // negative is backwards

const ENTRY_INTERVAL: Microns = 1000;
const ANGLE_INTERVAL: Radians = PI / 180.0;
const ANGLE_MAX: Radians = PI / 2.0;
const ANGLE_MIN: Radians = -ANGLE_MAX;

fn top_width(_: ParamSet) -> Microns {
	104 * 1000
}

fn mm_tenths_to_microns(mm10ths: u8) -> Microns {
	(mm10ths as Microns) * 100
}

pub fn raytrace(params: ParamSet) -> Performance {
	let mut traces = Vec::with_capacity(18960);

	let top_width = top_width(params);
	let mut entry: Microns = 0;
	while entry <= top_width {
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

const RI_AIR: u8 = 1;

#[derive(Clone, Copy, Default, Debug)]
struct Pos {
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
		let travel = self.y - down;
		self.y = down;
		travel
		// todo!("travel")
	}
}

fn snells(old_ri: u8, new_ri: u8, angle: Radians) -> (Snells, Radians) {
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
enum Snells {
	Refracted,
	Critical,
	Reflected,
}
