use paramset::ParamSet;

mod fitness;
mod genotype;
mod model;
mod mutation;
mod paramset;

fn main() {
	println!("min: {}", ParamSet::default());
	println!("max: {}", ParamSet::nth(dbg!(ParamSet::MAX_POSSIBILITIES)));
}
