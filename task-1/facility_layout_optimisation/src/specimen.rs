use crate::{generate_randomised_facilities, Dimensions, Facility, FacilityLayout};
use std::cmp::Ordering::Equal;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Specimen {
    pub facility: Facility,
    pub fitness: u64,
}

impl Specimen {
    pub fn new(facility: Facility, fitness: u64) -> Self {
        Specimen { facility, fitness }
    }
}

pub struct Population {
    pub specimens: Vec<Specimen>,
}

impl Population {
    pub fn fit_facilities(facility_population: Vec<Facility>, layout: &FacilityLayout) -> Self {
        Self {
            specimens: facility_population
                .into_iter()
                .map(|facility| {
                    let fitness = facility.calculate_fitness(layout);
                    Specimen::new(facility, fitness)
                })
                .collect(),
        }
    }

    pub fn select_by_tournament(&self, tournament_size: u64) -> Result<&Specimen, &'static str> {
        if tournament_size as usize > self.specimens.len() {
            return Err("The tournament size must be less than or equal the total specimen count.");
        }

        let rng = &mut rand::thread_rng();

        self.specimens
            .choose_multiple(rng, tournament_size as usize)
            .min_by(|first, second| first.fitness.cmp(&second.fitness))
            .ok_or("There are no specimens to choose from.")
    }

    pub fn select_by_roulette(&self) -> Result<&Specimen, &'static str> {
        let square_fitness_sum = self.specimens.iter().fold(0, |sum_acc, specimen| {
            sum_acc + specimen.fitness * specimen.fitness
        });

        let mut roulette_specimens: Vec<RouletteSpecimen> = self
            .specimens
            .iter()
            .map(|specimen| RouletteSpecimen {
                specimen,
                likelihood: (specimen.fitness * specimen.fitness) as f32
                    / (square_fitness_sum as f32),
                likelihood_bound: None,
            })
            .collect();

        // find Q = min likelihood + max likelihood
        // by using Q instead of the sum, the inverse likelihoods are not so flat
        let min_max_square_sum = roulette_specimens
            .iter()
            .min_by(|first, second| {
                first
                    .likelihood
                    .partial_cmp(&second.likelihood)
                    .unwrap_or(Equal)
            })
            .map(|min| min.likelihood)
            .unwrap_or(0.0)
            + roulette_specimens
                .iter()
                .max_by(|first, second| {
                    first
                        .likelihood
                        .partial_cmp(&second.likelihood)
                        .unwrap_or(Equal)
                })
                .map(|max| max.likelihood)
                .unwrap_or(0.0);

        // "inverse" the likelihoods
        for roulette_specimen in &mut roulette_specimens {
            roulette_specimen.likelihood = min_max_square_sum - roulette_specimen.likelihood
        }

        // TODO functionally?
        let mut current_likelihood_bound = 0.0;
        for mut roulette_specimen in &mut roulette_specimens {
            current_likelihood_bound += roulette_specimen.likelihood;
            roulette_specimen.likelihood_bound = Some(current_likelihood_bound);
        }

        // get the roulette guess
        let mut rng = rand::thread_rng();

        let guess = rng.gen_range(0.0..=current_likelihood_bound);

        roulette_specimens
            .iter()
            .find(|specimen| guess <= specimen.likelihood_bound.unwrap_or(0.0))
            .map(|roulette_specimen| roulette_specimen.specimen)
            .ok_or("No specimen has likelihood_bound as high as the guess.")
    }

    // TODO fix this...
    #[allow(clippy::too_many_arguments)]
    pub fn simulate_tournament<F>(
        population_size: u32,
        dimensions: &Dimensions,
        facility_layout: &FacilityLayout,
        selection_function: F,
        crossover_factor: f64,
        mutation_factor: f64,
        runs: u32,
        file_name: &str,
    ) -> Result<u64, &'static str>
    where
        F: Fn(&Population) -> Result<&Specimen, &'static str>,
    {
        // TODO fix this...
        #[allow(clippy::too_many_arguments)]
        fn simulate<F>(
            previous_population: Population,
            facility_layout: &FacilityLayout,
            crossover_factor: f64,
            mutation_factor: f64,
            selection_function: F,
            // TODO this should probably be a field in Population?
            max_machine: u64,
            runs: u32,
            runs_elapsed: u32,
            file_name: &str,
        ) -> Result<u64, &'static str>
        where
            F: Fn(&Population) -> Result<&Specimen, &'static str>,
        {
            // step 0. - write the simulation statistics
            let stats = Population::calculate_statistics(&previous_population.specimens)
                .unwrap_or((0, 0, 0.0, 0.0));

            let mut file = OpenOptions::new().append(true).open(file_name).unwrap();
            writeln!(file, "{},{},{},{}", stats.0, stats.1, stats.2, stats.3)
                .expect("Unable to write file");

            // check the exit condition
            if runs_elapsed == runs {
                return Ok(previous_population
                    .specimens
                    .iter()
                    .min_by(|first, second| first.fitness.cmp(&second.fitness))
                    .ok_or("TODO")?
                    .fitness);
            }

            // step 1. - selection
            let selection: Vec<&Specimen> = (0..previous_population.specimens.len())
                .map(|_x| selection_function(&previous_population))
                .into_iter()
                .collect::<Result<Vec<&Specimen>, &str>>()?;

            // step 2. - crossover
            // each specimen is chosen for crossover with a given probability
            // they are then connected into pairs
            // if there is an uneven amount of crossover specimens, the last one is just copied
            let mut new_population: Vec<Specimen> = Vec::new();
            let mut crossover_specimens: Vec<&Specimen> = Vec::new();
            let mut rng = rand::thread_rng();

            for specimen in selection {
                if rng.gen_bool(crossover_factor) {
                    crossover_specimens.push(specimen);
                } else {
                    new_population.push(specimen.clone());
                }
            }

            crossover_specimens.shuffle(&mut rng);

            if crossover_specimens.len() % 2 != 0 {
                new_population.push(crossover_specimens.pop().ok_or("TODO")?.clone());
            }

            // the actual crossover takes place here
            new_population.append(
                &mut crossover_specimens
                    .chunks_exact(2)
                    .flat_map(|crossover_chunk_iter| {
                        // TODO remove unwrap?
                        let [first, second]: [&Specimen; 2] =
                            crossover_chunk_iter.try_into().unwrap();
                        let result = first.facility.crossover(&second.facility);

                        Population::fit_facilities(vec![result.0, result.1], facility_layout)
                            .specimens
                    })
                    .collect(),
            );

            // step 3. - mutation
            // each specimen is mutated with a given probability
            for specimen in &mut new_population {
                specimen.facility.mutate(mutation_factor, max_machine);
            }

            // TODO shouldn't this be the first step?
            // step 3.5. - refit the population after mutation
            for specimen in &mut new_population {
                specimen.fitness = specimen.facility.calculate_fitness(facility_layout);
            }

            // step 4. - call the next iteration
            simulate(
                Population {
                    specimens: new_population,
                },
                facility_layout,
                crossover_factor,
                mutation_factor,
                selection_function,
                max_machine,
                runs,
                runs_elapsed + 1,
                file_name,
            )
        }

        let starting_population = Population::fit_facilities(
            generate_randomised_facilities(dimensions, population_size),
            facility_layout,
        );

        let max_machine = *starting_population
            .specimens
            .first()
            .and_then(|specimen| specimen.facility.find_max_machine())
            .expect("TODO");

        simulate(
            starting_population,
            facility_layout,
            crossover_factor,
            mutation_factor,
            selection_function,
            max_machine,
            runs,
            0,
            file_name,
        )
    }

    fn calculate_statistics(specimens: &[Specimen]) -> Result<(u64, u64, f32, f32), &'static str> {
        let best_fitness = specimens
            .iter()
            .min_by(|first, second| first.fitness.cmp(&second.fitness))
            .ok_or("TODO")?
            .fitness;
        let worst_fitness = specimens
            .iter()
            .max_by(|first, second| first.fitness.cmp(&second.fitness))
            .ok_or("TODO")?
            .fitness;
        let average_fitness = specimens
            .iter()
            .map(|specimen| specimen.fitness)
            .sum::<u64>() as f32
            / specimens.len() as f32;
        let standard_deviation = (specimens
            .iter()
            .map(|specimen| {
                let diff = average_fitness - specimen.fitness as f32;
                diff * diff
            })
            .sum::<f32>()
            / specimens.len() as f32)
            .sqrt();

        Ok((
            best_fitness,
            worst_fitness,
            average_fitness,
            standard_deviation,
        ))
    }
}

struct RouletteSpecimen<'a> {
    pub specimen: &'a Specimen,
    pub likelihood: f32,
    pub likelihood_bound: Option<f32>, // TODO this should maybe be calculated at creation
}
