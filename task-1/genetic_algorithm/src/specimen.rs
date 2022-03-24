use crate::{generate_randomised_facilities, Dimensions, Facility, FacilityLayout};

use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Specimen {
    pub facility: Facility,
    pub fitness: u64,
    likelihood: Option<f32>, // TODO this should maybe be calculated at creation?
    likelihood_bound: Option<f32>,
}

impl Specimen {
    pub fn new(facility: Facility, fitness: u64) -> Self {
        Specimen {
            facility,
            fitness,
            likelihood: None,
            likelihood_bound: None,
        }
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

    pub fn select_by_roulette(&mut self) -> Result<&Specimen, &'static str> {
        let total_fitness = self
            .specimens
            .iter()
            .fold(0, |sum_acc, specimen| sum_acc + specimen.fitness);

        // TODO functionally?
        for mut specimen in &mut self.specimens {
            specimen.likelihood = Some(1.0 - (specimen.fitness as f32) / (total_fitness as f32));
        }

        // normalize the likelihoods to sum up to 1
        let likelihood_sum = self.specimens.iter().fold(0.0, |likelihood_acc, specimen| {
            likelihood_acc + specimen.likelihood.unwrap_or(0.0)
        });

        let mut current_likelihood_bound = 0.0;
        for mut specimen in &mut self.specimens {
            let likelihood = specimen.likelihood.unwrap_or(0.0);

            specimen.likelihood = Some(likelihood / likelihood_sum);

            let likelihood = specimen.likelihood.unwrap_or(0.0);

            current_likelihood_bound += likelihood;
            specimen.likelihood_bound = Some(current_likelihood_bound);
        }

        // get the roulette guess
        let mut rng = rand::thread_rng();

        let guess = rng.gen::<f32>();

        self.specimens
            .iter()
            .find(|specimen| guess <= specimen.likelihood_bound.unwrap_or(0.0))
            .ok_or("No specimen has likelihood_bound as high as the guess.")
    }

    pub fn simulate_tournament(
        population_size: u32,
        dimensions: &Dimensions,
        facility_layout: &FacilityLayout,
        tournament_size: u64,
        crossover_factor: f64,
        mutation_factor: f64,
        runs: u32,
    ) -> Result<u64, &'static str> {
        fn simulate(
            previous_population: Population,
            facility_layout: &FacilityLayout,
            crossover_factor: f64,
            mutation_factor: f64,
            tournament_size: u64,
            // TODO this should probably be a field in Population?
            max_machine: u64,
            runs: u32,
            runs_elapsed: u32,
        ) -> Result<u64, &'static str> {
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
                .map(|_x| previous_population.select_by_tournament(tournament_size))
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
                tournament_size,
                max_machine,
                runs,
                runs_elapsed + 1,
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
            tournament_size,
            max_machine,
            runs,
            0,
        )
    }
}
