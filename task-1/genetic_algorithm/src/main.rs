use crate::facility::Facility;
use crate::facility_configuration::{Dimensions, FacilityConfig};
use crate::facility_layout::FacilityLayout;
use crate::flow_parser::parse_flows;
use crate::specimen::{Population, Specimen};

mod facility;
mod facility_configuration;
mod facility_layout;
mod flow_parser;
mod specimen;

fn main() {
    let easy_config = FacilityConfig::get_easy_config().unwrap();
    let population_size: u32 = 20;

    let facility_layout = parse_flows(easy_config.get_flow_path(), easy_config.get_cost_path());

    let facilities = generate_randomised_population(&easy_config.dimensions, population_size);
    let mut population = fit_population(facilities, &facility_layout);

    println!("{}", population.select_by_tournament(5).unwrap().fitness);
    println!("{}", population.select_by_roulette().unwrap().fitness);

    println!("---------- crossover: ----------");
    println!("{:?}", test_crossover());

    println!("---------- mutation: ----------");
    println!("{:?}", test_mutation());
}

fn generate_randomised_population(dimensions: &Dimensions, population_size: u32) -> Vec<Facility> {
    (0..population_size)
        .map(|_x| Facility::generate_randomised_facility(dimensions))
        .collect()
}

fn fit_population(facility_population: Vec<Facility>, layout: &FacilityLayout) -> Population {
    Population {
        specimens: facility_population
            .into_iter()
            .map(|facility| {
                let fitness = facility.calculate_fitness(layout);
                Specimen::new(facility, fitness)
            })
            .collect(),
    }
}

fn test_crossover() {
    let test_dimensions = Dimensions {
        width: 3,
        height: 3,
        machines: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
    };

    let test_population = generate_randomised_population(&test_dimensions, 2);
    let crossover = test_population[0].crossover(&test_population[1]);

    println!("first: {:?}", test_population[0]);
    println!("second: {:?}", test_population[1]);
    println!("crossover: {:?}", crossover);
}

fn test_mutation() {
    let test_dimensions = Dimensions {
        width: 3,
        height: 3,
        machines: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
    };

    let mut test_population = generate_randomised_population(&test_dimensions, 1);
    println!("before mutation: {:?}", test_population[0]);

    test_population[0].mutate(5, 8);
    println!("after mutation: {:?}", test_population[0]);
}
