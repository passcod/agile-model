use genevo::{ga::builder::EmptyGeneticAlgorithmBuilder, operator::prelude::*, prelude::*};

use fitness::AgileFitness;
use mutation::crossover::AgileCrossBreeder;
use paramset::ParamSet;

mod array;
mod builder;
mod fitness;
mod model;
mod mutation;
mod paramset;

const POPULATION_SIZE: usize = 200;
const GENERATION_LIMIT: u64 = 2000;
const NUM_INDIVIDUALS_PER_PARENTS: usize = 3;
const SELECTION_RATIO: f64 = 0.7;
const MUTATION_RATE: f64 = 0.05;
const REINSERTION_RATIO: f64 = 0.7;

fn main() {
	println!("min: {}", ParamSet::default());
	println!("max: {}", ParamSet::nth(ParamSet::MAX_POSSIBILITIES));

	let initial_population = build_population()
		.with_genome_builder(builder::RandomBuilder)
		.of_size(POPULATION_SIZE)
		.uniform_at_random();

	let alg: EmptyGeneticAlgorithmBuilder<ParamSet, _> = genetic_algorithm();
	let mut sim = simulate(
		alg.with_evaluation(AgileFitness)
			.with_selection(RouletteWheelSelector::new(
				SELECTION_RATIO,
				NUM_INDIVIDUALS_PER_PARENTS,
			))
			.with_crossover(AgileCrossBreeder)
			.with_mutation(BreederValueMutator::new(
				MUTATION_RATE,
				1,
				3,
				u8::MIN,
				u8::MAX,
			))
			.with_reinsertion(ElitistReinserter::new(
				AgileFitness,
				false,
				REINSERTION_RATIO,
			))
			.with_initial_population(initial_population)
			.build(),
	)
	.until(or(
		FitnessLimit::new(AgileFitness.highest_possible_fitness()),
		GenerationLimit::new(GENERATION_LIMIT),
	))
	.build();

	loop {
		let result = sim.step();
		match result {
			Ok(SimResult::Intermediate(step)) => {
				let evaluated_population = step.result.evaluated_population;
				let best_solution = step.result.best_solution;
				println!(
					"Step: generation: {}, average_fitness: {}, best fitness: {}, duration: {:?}, processing_time: {:?}",
					step.iteration,
					evaluated_population.average_fitness(),
					best_solution.solution.fitness,
					step.duration,
					step.processing_time
				);
				println!("{}", best_solution.solution.genome);
			}
			Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
				let best_solution = step.result.best_solution;
				println!("{}", stop_reason);
				println!(
					"Final result after {:?}: generation: {}, best solution with fitness {} found in generation {}, processing_time: {:?}",
					duration,
					step.iteration,
					best_solution.solution.fitness,
					best_solution.generation,
					processing_time
				);
				println!("{}", best_solution.solution.genome);
				break;
			}
			Err(error) => {
				println!("{}", error);
				break;
			}
		}
	}
}
