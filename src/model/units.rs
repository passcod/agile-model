pub type Microns = u64; // forwards from leftmost
pub type Radians = f64; // negative is backwards

pub fn mm_tenths_to_microns(mm10ths: u8) -> Microns {
	(mm10ths as Microns) * 100
}

/// Convert from model RI to real RI
pub fn model_ri_to_real_ri(model_ri: u8) -> f64 {
	f64::from(model_ri) * 0.01 + 0.99
}

pub const RI_AIR: u8 = 1;
