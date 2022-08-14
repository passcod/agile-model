use paramset::ParamSet;

mod paramset;

fn main() {
	println!("min: {}", ParamSet::default());
	println!("max: {}", ParamSet::nth(dbg!(ParamSet::MAX_POSSIBILITIES)));
}
