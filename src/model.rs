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
const ANGLE_MAX: Millirads = ((PI / 2.0) * 1000.0) as Millirads;
const ANGLE_MIN: Millirads = -ANGLE_MAX;

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
		let mut angle: Millirads = ANGLE_MIN;
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

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct Pos {
	/// Horizontal position from leftmost
	pub x: Microns,

	/// Vertical height
	pub y: Microns,

	/// Current RI
	pub ri: u8,

	/// Current direction (from normal)
	pub dir: Millirads,

	/// Vertical direction
	pub going_down: bool,
}

impl Pos {
	/// Recomputes directions from the RI change at a boundary.
	///
	/// Also does total internal reflection as needed.
	pub fn refract_into(&mut self, new_ri: u8) {
		todo!("refraction")
	}

	/// Takes vertical distances to boundaries above and below,
	/// outputs travel distance.
	///
	/// Also does reflection as needed.
	pub fn travel_to_next_boundary(&mut self, up: Microns, down: Microns) -> Microns {
		todo!("travel")
	}
}
