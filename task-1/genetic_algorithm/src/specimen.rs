use crate::{generate_randomised_facilities, Dimensions, Facility, FacilityConfig, FacilityLayout};

use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
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
        runs: u32,
    ) -> Result<u32, &'static str> {
        fn simulate(
            previous_population: Population,
            layout: &FacilityLayout,
            crossover_factor: f64,
            mutation_factor: f64,
            tournament_size: u64,
            total_runs: u32,
            runs_elapsed: u32,
        ) {
            if runs_elapsed == total_runs {
                return???
            }

            // step 1. - selection
            let selection = (0..previous_population.specimens.len())
                .map(|_x| previous_population.select_by_tournament(tournament_size)?)
                .collect();

            // step 2. - crossover
            // each specimen is chosen for crossover with a given probability
            // they are then connected into pairs
            // if there is an uneven amount of crossover specimens, the last one is just copied

            // TODO maybe pass this instead of creating in every iteration?
            let mut rng = rand::thread_rng();

            let mut new_population: Vec<Specimen> = Vec::new();
            let mut crossover_specimens: Vec<Specimen> = Vec::new();

            for specimen in selection {
                if rng.gen_bool(crossover_factor) {
                    crossover_specimens.push(specimen);
                } else {
                    new_population.push(specimen.clone());
                }
            }

            crossover_specimens.shuffle(&mut rng);

            if crossover_specimens.len() % 2 != 0 {
                new_population.push(crossover_specimens.pop()?.clone());
            }

            crossover_specimens.group_by(|specimen, _| specimen / )

            1
        }

        let starting_population = Population::fit_facilities(
            generate_randomised_facilities(dimensions, population_size),
            facility_layout,
        );

        simulate(runs, 0);
    }
}
