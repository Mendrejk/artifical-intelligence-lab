// TODO enable and fix all of those... Remember to run 'cargo clean' first
//#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

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
    // let config = FacilityConfig::get_easy_config().unwrap();
    // let config = FacilityConfig::get_flat_config().unwrap();
    let config = FacilityConfig::get_hard_config().unwrap();
    let population_size: u32 = 20;

    let facility_layout = parse_flows(config.get_flow_path(), config.get_cost_path());

    let facilities = generate_randomised_facilities(&config.dimensions, population_size);
    let population = Population::fit_facilities(facilities, &facility_layout);

    println!("{}", population.select_by_tournament(5).unwrap().fitness);
    println!("{}", population.select_by_roulette().unwrap().fitness);

    println!("---------- crossover: ----------");
    test_crossover();

    println!("---------- mutation: ----------");
    test_mutation();

    println!("-------- tournament: --------");
    // let specialised_tournament = |tournament_size| {
    //     move |population| Population::select_by_tournament(population, tournament_size)
    // };
    let specialised_tournament: fn(&Population) -> Result<&Specimen, &'static str> =
        |population| Population::select_by_tournament(population, 5);

    println!(
        "{:?}",
        Population::simulate_tournament(
            1000,
            &config.dimensions,
            &facility_layout,
            // Population::select_by_roulette,
            specialised_tournament,
            0.15,
            0.05,
            500
        )
    )
}

fn generate_randomised_facilities(dimensions: &Dimensions, population_size: u32) -> Vec<Facility> {
    (0..population_size)
        .map(|_x| Facility::generate_randomised_facility(dimensions))
        .collect()
}

fn test_crossover() {
    let test_dimensions = Dimensions {
        width: 3,
        height: 3,
        machines: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
    };

    let test_facilities = generate_randomised_facilities(&test_dimensions, 2);
    let crossover = test_facilities[0].crossover(&test_facilities[1]);

    println!("first: {:?}", test_facilities[0]);
    println!("second: {:?}", test_facilities[1]);
    println!("first crossover: {:?}", crossover.0);
    println!("second crossover: {:?}", crossover.1);
}

fn test_mutation() {
    let test_dimensions = Dimensions {
        width: 3,
        height: 3,
        machines: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
    };

    let mut test_facilities = generate_randomised_facilities(&test_dimensions, 1);
    println!("before mutation: {:?}", test_facilities[0]);

    test_facilities[0].mutate(0.15, 8);
    println!("after mutation: {:?}", test_facilities[0]);
}
