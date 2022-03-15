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
